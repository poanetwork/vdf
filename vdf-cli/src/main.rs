// Copyright 2018 Chia Network Inc and POA Networks Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![forbid(unsafe_code)]
#![allow(clippy::all)]

use clap::{Arg, ArgAction, Command};
use hex;
use std::{fs::File, io::Read};

use vdf::{InvalidProof, PietrzakVDFParams, VDFParams, WesolowskiVDFParams, VDF};

fn parse_proof_bytes(s: &str) -> Result<Vec<u8>, String> {
    if let Some(file_name) = s.strip_prefix('@') {
        let mut file = File::open(file_name)
            .map_err(|err| format!("Could not open {:?}: {}", file_name, err))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|err| format!("Could not read {:?}: {}", file_name, err))?;
        Ok(buf)
    } else {
        hex::decode(s).map_err(|err| format!("Invalid hex {:?}: {}", s, err))
    }
}

fn parse_hex_challenge(s: &str) -> Result<Vec<u8>, String> {
    hex::decode(s).map_err(|err| format!("Invalid hex {:?}: {}", s, err))
}

fn parse_proof_type(s: &str) -> Result<String, String> {
    if s == "pietrzak" || s == "wesolowski" {
        Ok(s.to_string())
    } else {
        Err("Invalid proof type".to_owned())
    }
}

fn main() {
    let matches = Command::new("vdf")
        .about("CLI to Verifiable Delay Functions")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help(
                    "Log verbosely to stderr. This command does not currently log anything, so \
                     this option currently has no affect.",
                )
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("TYPE")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("The type of proof to generate")
                .value_parser(parse_proof_type),
        )
        .arg(
            Arg::new("LENGTH")
                .short('l')
                .long("length")
                .value_name("BITS")
                .help("Length in bits of the discriminant (default: 2048)")
                .default_value("2048")
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(
            Arg::new("DISCRIMINANT_CHALLENGE")
                .required(true)
                .value_parser(parse_hex_challenge)
                .help("Hex-encoded challenge to derive the discriminant from"),
        )
        .arg(
            Arg::new("NUM_ITERATIONS")
                .required(true)
                .value_parser(clap::value_parser!(u64))
                .help("The number of iterations"),
        )
        .arg(
            Arg::new("PROOF")
                .value_parser(parse_proof_bytes)
                .required(false)
                .help(
                    "The hex-encoded proof, or @ followed by a file containing the proof as raw \
                     binary data",
                ),
        )
        .get_matches();

    let _verbose = matches.get_flag("verbose");

    let is_pietrzak = matches
        .get_one::<String>("TYPE")
        .map(|x| x == "pietrzak")
        .unwrap_or(false);

    let int_size_bits: u16 = *matches.get_one::<u16>("LENGTH").unwrap();

    let iterations = {
        let iterations = *matches.get_one::<u64>("NUM_ITERATIONS").unwrap();
        if is_pietrzak && (iterations & 1 != 0 || iterations < 66) {
            eprintln!("error: Number of iterations must be even and at least 66");
            std::process::exit(2);
        }
        iterations
    };

    let challenge = matches
        .get_one::<Vec<u8>>("DISCRIMINANT_CHALLENGE")
        .unwrap()
        .clone();

    let vdf: Box<dyn VDF> = if is_pietrzak {
        Box::new(PietrzakVDFParams(int_size_bits).new()) as _
    } else {
        Box::new(WesolowskiVDFParams(int_size_bits).new()) as _
    };

    if let Some(actual_proof) = matches.get_one::<Vec<u8>>("PROOF") {
        match vdf.verify(&challenge, iterations, actual_proof) {
            Ok(()) => println!("Proof is valid"),
            Err(InvalidProof) => {
                println!("Invalid proof");
                std::process::exit(1)
            }
        }
    } else {
        let proof = vdf
            .solve(&challenge, iterations)
            .expect("iterations should have been validated earlier");
        println!("{}", hex::encode(proof))
    }
}
