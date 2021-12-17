use aoc::prelude::*;
use nom::{
    bits::complete::{tag, take},
    branch::alt,
    combinator::map_opt,
    error::Error,
    multi::many_till,
    sequence::{pair, preceded},
    IResult, Parser,
};

#[derive(PartialEq, Eq, Debug)]
struct Packet {
    version: u8,
    op: Op,
}

impl Packet {
    pub fn new(version: u8, op: Op) -> Packet {
        Packet { version, op }
    }
}

impl Packet {
    pub fn checksum(&self) -> u64 {
        use Op::*;
        self.version as u64 +
        match &self.op {
            Sum(ps) | Product(ps) | Minimum(ps) | Maximum(ps) => ps.iter().map(|p| p.checksum()).sum(),
            Literal(_) => 0,
            Greater(a, b) | Less(a, b) | Equal(a, b) => a.checksum() + b.checksum(),
       }
    }

    pub fn eval(&self) -> u64 {
        use Op::*;
        match &self.op {
            Sum(ps) => ps.iter().map(|p| p.eval()).sum(),
            Product(ps) => ps.iter().map(|p| p.eval()).fold(1, |a, b| a * b),
            Minimum(ps) => ps.iter().map(|p| p.eval()).min().unwrap_or(0),
            Maximum(ps) => ps.iter().map(|p| p.eval()).max().unwrap_or(0),
            Literal(n) => *n,
            Greater(a, b) => (a.eval() > b.eval()) as u64,
            Less(a, b) => (a.eval() < b.eval()) as u64,
            Equal(a, b) => (a.eval() == b.eval()) as u64,
       }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Op {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(u64),
    Greater(Box<Packet>, Box<Packet>),
    Less(Box<Packet>, Box<Packet>),
    Equal(Box<Packet>, Box<Packet>),
}


// Bit stream error signature. Needs to be passed to nom turbofishes.
type E<'a> = Error<(&'a [u8], usize)>;

/// Parse packets given a buffer length in bits.
fn length_packets(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
    fn bits((array, offset): (&[u8], usize)) -> usize {
        array.len() * 8 - offset
    }

    let (mut input, n_bits): (_, usize) = preceded(tag(0, 1usize), take(15usize))(input)?;
    //                                                 ^ marker for bit count
    let target_bits = bits(input) - n_bits;
    let mut ret = Vec::new();

    while bits(input) > target_bits {
        let (rest, packet) = packet(input)?;
        input = rest;
        ret.push(packet);
    }

    Ok((input, ret))
}

/// Parse packets given a packet count.
fn count_packets(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
    let (mut input, count): (_, usize) = preceded(tag(1, 1usize), take(11usize))(input)?;
    //                                                ^ marker for packet count
    let mut ret = Vec::new();

    for _ in 0..count {
        let (rest, packet) = packet(input)?;
        input = rest;
        ret.push(packet);
    }

    Ok((input, ret))
}

fn packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    // Version is a 3-bit integer.
    let version = take(3usize);

    // Parse 1-prefixed 5-bit chunks until we hit the terminal 0-bit prefixed
    // chunk. Then merge the non-marker nybbles into the number.
    let number = many_till::<_, _, _, E, _, _>(
        preceded(tag(1, 1usize), take(4usize)),
        preceded(tag(0, 1usize), take(4usize)),
    )
    .map(|(xs, x): (Vec<u8>, u8)| {
        xs.iter()
            .chain(std::iter::once(&x))
            .fold(0u64, |a, &x| (a << 4) + x as u64)
    });

    // Must be a closure or we hit borrow checker badness.
    let packet_list = || alt((length_packets, count_packets));

    // Ditto. Convert a packet list of exactly two packets into a pair of
    // boxes.
    let packet_pair = || {
        map_opt(
            packet_list(),
            |mut list: Vec<Packet>| -> Option<(Box<Packet>, Box<Packet>)> {
                if list.len() == 2 {
                    let mut drain = list.drain(..);
                    Some((
                        Box::new(drain.next().unwrap()),
                        Box::new(drain.next().unwrap()),
                    ))
                } else {
                    None
                }
            },
        )
    };

    // Op parsing switchboard.
    let op = alt((
        preceded(tag(0, 3usize), packet_list()).map(|ps| Op::Sum(ps)),
        preceded(tag(1, 3usize), packet_list()).map(|ps| Op::Product(ps)),
        preceded(tag(2, 3usize), packet_list()).map(|ps| Op::Minimum(ps)),
        preceded(tag(3, 3usize), packet_list()).map(|ps| Op::Maximum(ps)),
        preceded(tag(4, 3usize), number).map(|n| Op::Literal(n)),
        preceded(tag(5, 3usize), packet_pair()).map(|(a, b)| Op::Greater(a, b)),
        preceded(tag(6, 3usize), packet_pair()).map(|(a, b)| Op::Less(a, b)),
        preceded(tag(7, 3usize), packet_pair()).map(|(a, b)| Op::Equal(a, b)),
        //           ^ op marker
    ));

    // Bring it all together into a packet.
    pair(version, op)(input).map(|(a, (x, y))| (a, Packet::new(x, y)))
}

fn parse(input: &[u8]) -> IResult<&[u8], Packet> {
    // Wrapper for nom bit parsing.
    nom::bits::<_, _, E, _, _>(packet)(input)
}

fn main() {
    let data = hex_to_bytes(stdin_string());
    let packet = parse(&data[..]).unwrap().1;
    println!("{}", packet.checksum());
    println!("{}", packet.eval());
}
