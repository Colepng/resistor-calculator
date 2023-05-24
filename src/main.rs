use std::{
    io::{self, Write},
    string::ParseError,
};
// fn main() -> io::Result<()> {

// enum that repersents if the program in running in Interactive mode or not
enum Mode {
    Interactive,
    CLI,
}

struct Resistor {
    // num_of_bands: u8,
    bands: [Colours; 6],
    digit1: u8,
    digit2: u8,
    digit3: Option<u8>,
    multiplier: f64,
    tolerance: f64,
}

impl Resistor {
    fn new(bands: [Colours; 6]) -> Self {
        let mut resistor = Resistor {
            bands: bands,
            digit1: 0,
            digit2: 0,
            digit3: None,
            multiplier: 0.0,
            tolerance: 20.0,
        };
        let mut high_precision: bool = false;
        // TODO: change 4 to const, 4 repersents the postion where if a band were there the
        // resistor would have 3 digit.
        if bands[4] != Colours::Blank {
            high_precision = true;
        }
        if !high_precision {
            resistor.digit1 = bands[0].as_digit();
            resistor.digit2 = bands[1].as_digit();
            resistor.multiplier = bands[2].as_multiplier();
            // resistor.tolerance = bands[3].as_tolerance().unwrap();
        } else {
            resistor.digit1 = bands[0].as_digit();
            resistor.digit2 = bands[1].as_digit();
            resistor.digit3 = Some(bands[2].as_digit());
            resistor.multiplier = bands[3].as_multiplier();
            resistor.tolerance = bands[4].as_tolerance().unwrap();
        }

        resistor
    }

    fn calculate(&self) -> f64 {
        let mut high_precision: bool = false;
        // TODO: change 4 to const, 4 repersents the postion where if a band were there the
        // resistor would have 3 digit.
        if self.bands[4] != Colours::Blank {
            high_precision = true;
        }
        let total: f64 = if !high_precision {
            (self.digit1 * 10 + self.digit2) as f64 * self.multiplier
        } else {
            (self.digit1 * 100 + self.digit2 * 10 + self.digit3.unwrap()) as f64 * self.multiplier
        };
        total
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Colours {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Gray,
    White,
    Gold,
    Silver,
    Blank,
}

impl TryFrom<String> for Colours {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match &*value.to_ascii_lowercase() {
            "black" => Ok(Self::Black),
            "brown" => Ok(Self::Brown),
            "red" => Ok(Self::Red),
            "orange" => Ok(Self::Orange),
            "yellow" => Ok(Self::Yellow),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            "violet" => Ok(Self::Violet),
            "gray" => Ok(Self::Gray),
            "white" => Ok(Self::White),
            "gold" => Ok(Self::Gold),
            "silver" => Ok(Self::Silver),
            _ => Err("Not a valid colour"),
        }
    }
}

impl Colours {
    fn as_digit(&self) -> u8 {
        match self {
            Self::Blank => 0,
            Self::Black => 0,
            Self::Brown => 1,
            Self::Red => 2,
            Self::Orange => 3,
            Self::Yellow => 4,
            Self::Green => 5,
            Self::Blue => 6,
            Self::Violet => 7,
            Self::Gray => 8,
            Self::White => 9,
            Self::Gold => 0,
            Self::Silver => 0,
        }
    }

    fn as_multiplier(&self) -> f64 {
        match self {
            Self::Blank => 1.0,
            Self::Black => 1.0,
            Self::Brown => 10.0,
            Self::Red => 100.0,
            Self::Orange => 1000.0,
            Self::Yellow => 10000.0,
            Self::Green => 100000.0,
            Self::Blue => 1000000.0,
            Self::Violet => 10000000.0,
            Self::Gray => 100000000.0,
            Self::White => 1000000000.0,
            Self::Gold => 0.1,
            Self::Silver => 0.01,
        }
    }

    fn as_tolerance(&self) -> Option<f64> {
        match self {
            Self::Blank => None,
            Self::Black => None,
            Self::Brown => Some(1.0),
            Self::Red => Some(2.0),
            Self::Orange => None,
            Self::Yellow => None,
            Self::Green => Some(0.5),
            Self::Blue => Some(0.25),
            Self::Violet => Some(0.1),
            Self::Gray => Some(0.05),
            Self::White => None,
            Self::Gold => Some(5.0),
            Self::Silver => Some(10.0),
        }
    }
}

fn main() {
    print!("How many bands does the resistor have: ");
    // flush stdout so that the prompt will be printed before reading from stdin
    let _ = io::stdout().flush();

    let mut buffer: String = String::new();
    let _ = io::stdin().read_line(&mut buffer);

    let num_of_bands: u8 = buffer.trim().parse::<u8>().unwrap_or(4);
    let mut bands: [Colours; 6] = [Colours::Blank; 6];

    for i in 1..=num_of_bands {
        print!("What color is band{i}: ");
        let _ = io::stdout().flush();

        let mut input: String = String::new();
        let _ = io::stdin().read_line(&mut input);
        input.truncate(input.trim().len());
        bands[(i - 1) as usize] = input.try_into().unwrap();
    }
    let resistor = Resistor::new(bands);
    println!("{} Ω ±{}%", resistor.calculate(), resistor.tolerance);
}
