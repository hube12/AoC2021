use aoc_2021::{Day, Solution1, Solution2};
use std::convert::TryFrom;
use std::str::Chars;

#[derive(Default)]
pub struct Day16;

impl Day for Day16 {}

fn convert_to_binary_from_hex(hex: &str) -> anyhow::Result<String> {
    hex.chars().map(to_binary).collect::<Result<_, _>>()
}

fn to_binary(c: char) -> anyhow::Result<&'static str> {
    Ok(match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => {
            return Err(anyhow::Error::msg("Not a valid Hex char"));
        }
    })
}

type Version = u8;

enum Type {
    Literal,
    Operator(OperatorType, OperatorConstaint),
}

enum OperatorType {
    Sum,
    Product,
    Mimimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<usize> for OperatorType {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Mimimum,
            3 => OperatorType::Maximum,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::EqualTo,
            _ => {
                return Err(anyhow::Error::msg("Not a valid operator type"));
            }
        })
    }
}

enum OperatorConstaint {
    OperatorLength(usize),
    OperatorSubPacket(usize),
}

#[derive(Debug)]
struct Packet {
    version: Version,
    content: usize,
}

impl Packet {
    fn new(version: Version, content: usize) -> Packet {
        Packet { version, content }
    }
}

fn parse_header(iter: &mut Chars) -> anyhow::Result<(Version, Type, usize)> {
    let version = parse_number::<3>(iter)?;
    let r#type = parse_number::<3>(iter)?;
    let mut size = 6;
    let r#type = match r#type {
        4 => Type::Literal,
        x => {
            match iter
                .next()
                .ok_or(anyhow::Error::msg("Missing char for header type"))?
            {
                '0' => {
                    let total_length = parse_number::<15>(iter)?;
                    size += 16;
                    Type::Operator(
                        OperatorType::try_from(x)?,
                        OperatorConstaint::OperatorLength(total_length),
                    )
                }
                '1' => {
                    let nbr_packets = parse_number::<11>(iter)?;
                    size += 12;
                    Type::Operator(
                        OperatorType::try_from(x)?,
                        OperatorConstaint::OperatorSubPacket(nbr_packets),
                    )
                }
                _ => {
                    return Err(anyhow::Error::msg("Not a valid binary"));
                }
            }
        }
    };
    Ok((version as u8, r#type, size))
}

fn parse_number<const N: usize>(iter: &mut Chars) -> anyhow::Result<usize> {
    let mut res = 0usize;
    for n in (0..N).rev() {
        match iter.next().ok_or(anyhow::Error::msg(format!(
            "Missing char for number at index {}",
            N - n
        )))? {
            '0' => {}
            '1' => {
                res |= 1 << n;
            }
            _ => {
                return Err(anyhow::Error::msg("Not a valid binary"));
            }
        }
    }
    Ok(res)
}

fn parse_literal(iter: &mut Chars) -> anyhow::Result<(usize, usize)> {
    let mut final_number:usize=0;
    let mut size = 0;
    loop {
        let not_last_group = iter
            .next()
            .ok_or(anyhow::Error::msg("Missing char for last group"))?;
        let number = parse_number::<4>(iter)?;
        size += 5;
        final_number+=number;
        match not_last_group {
            '0' => {
                break;
            }
            '1' => {
                final_number*=16;
                continue;
            }
            _ => {
                return Err(anyhow::Error::msg("Not a valid binary"));
            }
        }
    }
    Ok((final_number, size))
}

fn parse_packet<const REDUCE:bool>(iter: &mut Chars, deep: usize) -> anyhow::Result<(Vec<Packet>, usize)> {
    let (version, r#type, header_size) = parse_header(iter)?;
    match r#type {
        Type::Literal => {
            let (literal, size) = parse_literal(iter)?;
            log::trace!(
                "{}Got literal with value {} and size {}",
                "\t".repeat(deep),
                literal,
                size + header_size
            );
            Ok((vec![Packet::new(version, literal)], size + header_size))
        }
        Type::Operator(operator_type, operator_constraint) => {
            let (mut packets, size) = match operator_constraint {
                OperatorConstaint::OperatorLength(mut n) => {
                    log::trace!("{}Got Op Packet with length {}", "\t".repeat(deep), n);
                    let mut res = vec![];
                    let mut packet_size = header_size;
                    loop {
                        let (packets, s) = parse_packet::<REDUCE>(iter, deep + 1)?;
                        packet_size += s;
                        if n < s {
                            return Err(anyhow::Error::msg(
                                "It's not possible for operator length to go negative",
                            ));
                        }
                        n -= s;
                        for packet in packets {
                            res.push(packet);
                        }
                        if n == 0 {
                            break;
                        }
                    }
                    (res, packet_size)
                }
                OperatorConstaint::OperatorSubPacket(n) => {
                    log::trace!("{}Got Op Packet with {} sub packets", "\t".repeat(deep), n);
                    let mut res = vec![];
                    let mut packet_size = header_size;
                    for _ in 0..n {
                        let (packets, s) = parse_packet::<REDUCE>(iter, deep + 1)?;
                        packet_size += s;
                        for packet in packets {
                            res.push(packet);
                        }
                    }
                    (res, packet_size)
                }
            };
            if REDUCE{
                let packet = match operator_type {
                    OperatorType::Sum => Packet::new(version, packets.iter().map(|x| x.content).sum()),
                    OperatorType::Product => Packet::new(
                        version,
                        packets
                            .iter()
                            .map(|x| x.content)
                            .reduce(|a, b| a * b)
                            .unwrap_or(0),
                    ),
                    OperatorType::Maximum => Packet::new(
                        version,
                        packets.iter().map(|x| x.content).max().unwrap_or(0),
                    ),
                    OperatorType::Mimimum => Packet::new(
                        version,
                        packets.iter().map(|x| x.content).min().unwrap_or(0),
                    ),
                    OperatorType::GreaterThan => Packet::new(
                        version,
                        if packets
                            .get(0)
                            .ok_or(anyhow::Error::msg("Missing packet 0"))?
                            .content
                            > packets
                            .get(1)
                            .ok_or(anyhow::Error::msg("Missing packet 1"))?
                            .content
                        {
                            1
                        } else {
                            0
                        },
                    ),
                    OperatorType::LessThan => Packet::new(
                        version,
                        if packets
                            .get(0)
                            .ok_or(anyhow::Error::msg("Missing packet 0"))?
                            .content
                            <packets
                            .get(1)
                            .ok_or(anyhow::Error::msg("Missing packet 1"))?
                            .content
                        {
                            1
                        } else {
                            0
                        },
                    ),
                    OperatorType::EqualTo => Packet::new(
                        version,
                        if packets
                            .get(0)
                            .ok_or(anyhow::Error::msg("Missing packet 0"))?
                            .content
                            == packets
                            .get(1)
                            .ok_or(anyhow::Error::msg("Missing packet 1"))?
                            .content
                        {
                            1
                        } else {
                            0
                        },
                    ),
                };
                Ok((vec![packet], size))
            }else{
                packets.insert(0,Packet::new(version,0));
                Ok((packets,size))
            }

        }
    }
}

fn parse<const REDUCE:bool>(bin: &str) -> anyhow::Result<Vec<Packet>> {
    let mut iter = bin.chars();
    let (packets, _) = parse_packet::<REDUCE>(&mut iter, 0)?;
    if iter.any(|x| x != '0') {
        return Err(anyhow::Error::msg("Not valid packet parsed"));
    }
    Ok(packets)
}

impl Solution1 for Day16 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let bits = lines.join("");
        let bin = convert_to_binary_from_hex(bits.as_str())?;
        let packets = parse::<false>(bin.as_str())?;
        Ok(packets
            .iter()
            .map(|x| x.version as usize)
            .sum::<usize>()
            .to_string())
    }
}

impl Solution2 for Day16 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let bits = lines.join("");
        let bin = convert_to_binary_from_hex(bits.as_str())?;
        let packets =  parse::<true>(bin.as_str())?;
        Ok(packets.first().ok_or(anyhow::Error::msg("Missing a packet"))?.content.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day16").unwrap();
        Ok(assert_eq!(
            Day16::default().run_solution1(lines)?,
            String::from("889")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day16").unwrap();
        Ok(assert_eq!(
            Day16::default().run_solution1(lines)?,
            String::from("31")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day16").unwrap();
        Ok(assert_eq!(
            Day16::default().run_solution2(lines)?,
            String::from("739303923668")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day16").unwrap();
        Ok(assert_eq!(
            Day16::default().run_solution2(lines)?,
            String::from("54")
        ))
    }
}
