use aoc_2021::{Day, Solution1, Solution2};
use std::ops::Sub;

#[derive(Default)]
pub struct Day7;

impl Day for Day7 {}

//  Next approximate gm computed from a non-member point p
fn nxnonmember<T: Copy + PartialOrd + std::fmt::Display>(points: &[Vec<T>], p: &[f64]) -> Vec<f64>
where
    f64: From<T>,
{
    let mut vsum = vec![0_f64; points.len()];
    let mut recip = 0_f64;
    for point in points {
        let mag = euclidean_dist(point, p).sqrt();
        if !mag.is_normal() {
            continue;
        } // zero distance, safe to ignore

        let rec = 1.0_f64 / mag;
        let weights: Vec<f64> = point.iter().map(|&x| rec * f64::from(x)).collect();
        vsum.iter_mut().zip(&weights).for_each(|(x, &vi)| *x += vi);

        recip += rec // add separately the reciprocals
    }
    let s: f64 = 1.0 / recip;
    vsum.iter_mut().for_each(|x| *x *= s);
    vsum
}

// simple multidimensional arithmetic mean
fn acentroid<T: Copy + PartialOrd + std::fmt::Display>(points: &[Vec<T>]) -> Vec<f64>
where
    f64: From<T>,
{
    // all the centres on each dim
    let mut centres: Vec<f64> = vec![0_f64; points.len()];
    for point in points {
        // centres + points on all dim
        centres
            .iter_mut()
            .zip(point)
            .for_each(|(x, &vi)| *x += f64::from(vi))
    }
    let s: f64 = 1.0 / (points.len() as f64);
    centres.iter_mut().for_each(|x| *x *= s);
    centres
}

/// Non Squared euclidean distance
fn euclidean_dist<T: Copy + PartialOrd + std::fmt::Display>(v: &[T], p: &[f64]) -> f64
where
    f64: From<T>,
{
    v.iter()
        .zip(p)
        .map(|(&xi, &vi)| (f64::from(xi) - vi).powi(2))
        .sum::<f64>()
}

fn geometric_median<T: Copy + PartialOrd + std::fmt::Display>(v: &[Vec<T>], eps: f64) -> Vec<f64>
where
    f64: From<T>,
{
    let eps2: f64 = eps.powi(2);
    let mut point: Vec<f64> = acentroid(v);
    loop {
        // vector iteration till accuracy eps is reached
        let nextp: Vec<f64> = nxnonmember(v, &point);
        if euclidean_dist::<f64>(nextp.as_slice(), &point) < eps2 {
            return nextp;
        }; // termination
        point = nextp
    }
}

fn calculate_dist<T: Copy + PartialOrd + std::fmt::Display>(v: &[Vec<T>], p: &[T]) -> f64
where
    f64: From<T>,
{
    v.iter()
        .map(|x| {
            x.iter()
                .zip(p)
                .map(|(&vi, &pi)| (f64::from(vi) - f64::from(pi)).abs())
                .sum::<f64>()
        })
        .sum()
}

fn dist(v: &Vec<usize>, p: usize) -> usize {
    v.iter()
        .map(|&x| ((p as isize).sub(x as isize)).abs() as usize)
        .sum()
}

#[allow(dead_code)]
fn dumb_solution(lines: Vec<String>) -> anyhow::Result<String> {
    let crabs: Vec<usize> = lines
        .first()
        .ok_or(anyhow::Error::msg("Missing a line"))?
        .split(',')
        .map(|x| usize::from_str_radix(x, 10))
        .collect::<Result<_, _>>()?;
    let min = *crabs
        .iter()
        .min()
        .ok_or(anyhow::Error::msg("Missing an element"))?;
    let max = *crabs
        .iter()
        .max()
        .ok_or(anyhow::Error::msg("Missing an element"))?;
    let mut min_dist = usize::MAX;
    let mut _pos_min = 0usize;
    for pos in min..max {
        let dist = dist(&crabs, pos);
        if dist < min_dist {
            min_dist = dist;
            _pos_min = pos;
        }
    }
    dbg!(_pos_min);
    Ok(min_dist.to_string())
}

impl Solution1 for Day7 {
    fn run_solution1(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let crabs: Vec<Vec<f64>> = lines
            .first()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .split(',')
            .map(|x| usize::from_str_radix(x, 10).map(|x| vec![x as f64]))
            .collect::<Result<_, _>>()?;
        let r = geometric_median(&crabs, 0.001);
        let first = r[0].round(); // we don't consider geometric median that are not integer
        let dist = calculate_dist(&crabs, &[first as f64]);
        Ok(dist.to_string())
    }
}

fn dist_arithmetic_suite(v: &Vec<usize>, p: usize) -> usize {
    v.iter()
        .map(|&x| ((p as isize).sub(x as isize)).abs() as usize)
        .map(|x| (x + 1) * x / 2)
        .sum()
}

impl Solution2 for Day7 {
    fn run_solution2(&self, lines: Vec<String>) -> anyhow::Result<String> {
        let crabs: Vec<usize> = lines
            .first()
            .ok_or(anyhow::Error::msg("Missing a line"))?
            .split(',')
            .map(|x| usize::from_str_radix(x, 10))
            .collect::<Result<_, _>>()?;
        let min = *crabs
            .iter()
            .min()
            .ok_or(anyhow::Error::msg("Missing an element"))?;
        let max = *crabs
            .iter()
            .max()
            .ok_or(anyhow::Error::msg("Missing an element"))?;
        let mut min_dist = usize::MAX;
        let mut _pos_min = 0usize;
        for pos in min..max {
            let dist = dist_arithmetic_suite(&crabs, pos);
            if dist < min_dist {
                min_dist = dist;
                _pos_min = pos;
            }
        }
        Ok(min_dist.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2021::collect_file;
    use aoc_2021::Part::{Part1, Part2, Test};

    #[test]
    fn solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part1, "Day7").unwrap();
        Ok(assert_eq!(
            Day7::default().run_solution1(lines)?,
            String::from("343468")
        ))
    }

    #[test]
    fn test_solution1() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day7").unwrap();
        Ok(assert_eq!(
            Day7::default().run_solution1(lines)?,
            String::from("37")
        ))
    }

    #[test]
    fn solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Part2, "Day7").unwrap();
        Ok(assert_eq!(
            Day7::default().run_solution2(lines)?,
            String::from("96086265")
        ))
    }

    #[test]
    fn test_solution2() -> anyhow::Result<()> {
        let lines: Vec<String> = collect_file(Test, "Day7").unwrap();
        Ok(assert_eq!(
            Day7::default().run_solution2(lines)?,
            String::from("168")
        ))
    }

    #[test]
    fn test_centroid() {
        #[rustfmt::skip]
            let pts: Vec<Vec<f64>> = vec![vec![0.0000000002328306435454494, 0.0013732912478161552, 0.008178711166522135, 0.14370727555653673, 0.0047912599962245395, 0.01570129417083166, 0.00796508811974661, 0.039550781464413376, 0.01977539084862201, 0.0808105470702003, 0.12503051775273377, 0.16052246109558155, 0.10180664081042323, 0.001525879138370101, 0.18072509780492396], vec![0.1251983644323431, 0.26562500010913936, 0.032043457249159246, 0.06835937520099833, 0.12512207048706614, 0.1339111329829734, 0.0010070803104866854, 0.0664672853581294, 0.01763916038086677, 0.1252593995885647, 0.1330718995849267, 0.000015259021886038118, 0.008178711166522135, 0.06274414082861313, 0.17968750014915713], vec![0.13398742692825039, 0.0010528566776528692, 0.03179931662427293, 0.10156250018553692, 0.016296386943992047, 0.2129211427118065, 0.06250000020372681, 0.0029296877314664016, 0.05178833028683982, 0.005371093980329533, 0.12501525896367838, 0.004150390855897967, 0.15722656265961632, 0.14868164078859536, 0.06274414082861313], vec![0.021484375222826202, 0.03912353537086233, 0.0664672853581294, 0.1291503907976903, 0.14331054704109647, 0.0039672853872332325, 0.0645141603590389, 0.03126525900733412, 0.046890259000058165, 0.08062744160153557, 0.01710510276392796, 0.032608032444208845, 0.004943847886778485, 0.0000000002328306435454494, 0.12792968767325874], vec![0.0024414064816937753, 0.15823364273727236, 0.09573364276637619, 0.0957183839773208, 0.07922363300843926, 0.015625000225554686, 0.06848144551344149, 0.007339477768475433, 0.06826782246666596, 0.20071411146749085, 0.03906250021464075, 0.13525390641984814, 0.12599182146322363, 0.00027465843582774585, 0.16029357925975063], vec![0.0026855471065800884, 0.10156250018553692, 0.06250000020372681, 0.12648010271299626, 0.015747070537997843, 0.004211426012119546, 0.00976562522828317, 0.008544922103851604, 0.18775939955946086, 0.14752197282038537, 0.25833129894066076, 0.024475097877683538, 0.007843017807303454, 0.00012207054527380013, 0.023696899635858415], vec![0.03128051779638952, 0.15972900406470103, 0.025421142799118, 0.12744140642348611, 0.008071899643134373, 0.020523071512336344, 0.015686035381776264, 0.037414550996658136, 0.024169922096575647, 0.0000000002328306435454494, 0.01567077659272087, 0.07037353535631041, 0.13427734392030288, 0.15625000016007107, 0.12500000017462298], vec![0.1791992188993845, 0.06411743184359864, 0.15237426774000085, 0.09376525897823029, 0.2506408692567419, 0.003952026598177838, 0.03149414084316504, 0.1294250490006874, 0.25781250011277734, 0.1258239747836143, 0.14953613297569746, 0.03125000021827873, 0.1328430177490958, 0.017120361552983354, 0.027633667212150215], vec![0.15747070328450263, 0.001129150622929842, 0.001953125231921149, 0.12568664568211574, 0.017028808818650987, 0.16029357925975063, 0.004150390855897967, 0.21905517591207513, 0.25892639171382115, 0.004669189683781383, 0.0004882814826032699, 0.00012207054527380013, 0.08071899433586793, 0.1623535157822289, 0.09381103534539648], vec![0.016113281475327312, 0.19963073744455784, 0.1347656251700755, 0.08187866230407792, 0.04785156271054802, 0.24830627453126652, 0.18798828139529178, 0.001525879138370101, 0.16253662125089363, 0.27151489268452167, 0.26959228526354195, 0.20071411146749085, 0.18994140639438228, 0.15435791031720214, 0.12512207048706614], vec![0.0003662111701601133, 0.1884765626450644, 0.0029602053095771907, 0.00038146995921550786, 0.06378173848437996, 0.07812500019645086, 0.25022888195224624, 0.004028320543454811, 0.034179687716914486, 0.07824707050889401, 0.0031127932001311365, 0.06890869160699253, 0.004440307847950464, 0.04785156271054802, 0.12710571306426743], vec![0.007843017807303454, 0.11097717303271537, 0.039581299042524165, 0.0000000002328306435454494, 0.008575439681962393, 0.01971435569240043, 0.01954650901279109, 0.03125000021827873, 0.14062500016734703, 0.03344726584225555, 0.0010528566776528692, 0.002136230700585884, 0.004531860582282832, 0.003906250231011654, 0.007858276596358849], vec![0.03149414084316504, 0.028701782446027835, 0.2500000001164153, 0.011962890852259989, 0.0000000002328306435454494, 0.06359863301571522, 0.12503051775273377, 0.25025939953035703, 0.06599426289741217, 0.02624511740810931, 0.21191406263415047, 0.12597656267416824, 0.000305176013938535, 0.012695312726918928, 0.14845275895276444], vec![0.0019989015990873327, 0.04887390157725946, 0.0012207033572622095, 0.015869140850441, 0.2562255860510163, 0.25415039073948265, 0.016616821514155333, 0.04785156271054802, 0.12506103533084456, 0.007934570541635821, 0.00976562522828317, 0.06317138692216417, 0.0004882814826032699, 0.03909301779275154, 0.1353149415760697], vec![0.0021057131224750947, 0.10705566424547897, 0.000732422107489583, 0.008453369369519237, 0.0645141603590389, 0.015655517803665475, 0.12597656267416824, 0.0332946779517016, 0.00619506858932084, 0.037109375215550244, 0.03222656271782398, 0.01603698753005034, 0.1353302003651251, 0.0650634767650331, 0.1330718995849267]];
        let centroid = acentroid(&pts);
        // sum of all euclidean distances
        let dist = pts
            .iter()
            .map(|p| {
                // simple euclidean distance
                p.iter()
                    .zip(&centroid)
                    .map(|(&xi, &vi)| (f64::from(xi) - f64::from(vi)).powi(2))
                    .sum::<f64>()
                    .sqrt()
            })
            .sum::<f64>();
        assert_eq!(dist, 4.14556218326653_f64);
    }
}
