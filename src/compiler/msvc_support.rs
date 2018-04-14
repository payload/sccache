extern crate encoding_rs;
use self::encoding_rs::*;

use compiler::args::*;

use std::str;
use std::fs::File;
use std::io::prelude::*;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

type MSVCArgItem = ArgumentItem<MSVCArgAttribute>;

#[derive(Debug)]
pub struct CliCalls {
    pub compiler_calls: Vec<Vec<OsString>>,
    pub after_calls: Vec<Vec<OsString>>,
}

#[derive(Debug)]
pub struct ReadArgs {
    input_args: Vec<PathBuf>,
    output_arg: Option<PathBuf>,
    unknown_flags: Vec<OsString>,
    pdb_arg: Option<PathBuf>,
    args: HashMap<MSVCArgAttribute, Vec<Argument>>,
}

impl ReadArgs {
    pub fn new() -> Self {
        ReadArgs {
            input_args: vec![],
            output_arg: None,
            unknown_flags: vec![],
            pdb_arg: None,
            args: HashMap::new(),
        }
    }

    fn create_input_output_pairs(&self) -> Vec<(PathBuf, PathBuf)> {
        if let Some(ref output_arg_path) = self.output_arg {
            let output_str = output_arg_path.to_string_lossy();
            if output_str.ends_with("\\") {
                self.input_args
                    .iter()
                    .map(|input| {
                        let input_path = input.clone();
                        let obj_path = Path::new(input).with_extension("obj");
                        let file_name = obj_path.file_name().unwrap();
                        let mut output_path = output_arg_path.clone();
                        output_path.push(file_name);
                        (input_path, output_path)
                    })
                    .collect()
            } else {
                assert!(self.input_args.len() == 1);
                let input_path = self.input_args.first().unwrap().clone();
                let output_path = output_arg_path.clone();
                vec![(input_path, output_path)]
            }
        } else {
            self.input_args
                .iter()
                .map(|input| {
                    let input_path = input.clone();
                    let output_path = input.as_path().with_extension("obj");
                    (input_path, output_path)
                })
                .collect()
        }
    }

    fn multiple_inputs(&self) -> bool {
        self.input_args.len() > 1
    }

    fn create_pdb_arg(&self, output_path: &PathBuf) -> Option<OsString> {
        if self.args.contains_key(&DebugInfo) {
            let mut pdb_arg = OsString::from("-Fd");
            if self.multiple_inputs() || self.pdb_arg == None {
                pdb_arg.push(output_path.as_os_str());
                pdb_arg.push(".pdb");
            } else {
                pdb_arg.push(self.pdb_arg.clone().unwrap());
            }
            Some(pdb_arg)
        } else {
            None
        }
    }

    fn create_compiler_call(&self, input_path: &PathBuf, output_path: &PathBuf) -> Vec<OsString> {
        let mut args = vec![];

        let input_arg = OsString::from(input_path.as_os_str());
        args.push(input_arg);

        let mut output_arg = OsString::from("-Fo");
        output_arg.push(output_path.as_os_str());
        args.push(output_arg);

        if let Some(pdb_arg) = self.create_pdb_arg(&output_path) {
            args.push(pdb_arg);
        }

        let mut other_args = self.args.iter().collect::<Vec<_>>();
        other_args.sort_by_key(|&(k, _)| k);
        for &(_, ref args_vec) in other_args.iter() {
            for arg in args_vec.iter() {
                args.extend_from_slice(&argument_to_os_string(arg));
            }
        }

        args.extend_from_slice(&self.unknown_flags);

        args
    }

    pub fn create_cli_calls(&self) -> CliCalls {
        let compiler_calls = self.create_input_output_pairs()
            .iter()
            .map(|&(ref input_path, ref output_path)| {
                self.create_compiler_call(input_path, output_path)
            })
            .collect();
        let after_calls = vec![self.handle_shared_pdb()];
        CliCalls {
            compiler_calls: compiler_calls,
            after_calls: after_calls,
        }
    }

    fn handle_shared_pdb(&self) -> Vec<OsString> {
        let debug_info = self.args.contains_key(&DebugInfo);
        let multiple_inputs = self.multiple_inputs();
        match (debug_info, multiple_inputs, &self.pdb_arg) {
            (true, true, &Some(ref pdb_path)) => vec![
                OsString::from("touch"),
                OsString::from(pdb_path.as_os_str()),
            ],
            (true, _, &None) => vec![OsString::from("touch"), OsString::from("v140.pdb")],
            _ => vec![],
        }
    }

    pub fn read_cli_args(&mut self, args_os_strs: &[OsString]) {
        let args_strs = args_os_strs
            .iter()
            .cloned()
            .map(|arg| String::from(arg.to_string_lossy()))
            .collect::<Vec<_>>();
        let args = args_vec_to_args(&args_strs);
        self.read_args(&args);
    }

    pub fn read_rsp_path(&mut self, rsp_path: &str) {
        let rsp_str = rsp_file_read_into_string(&rsp_path);
        let args_vec = string_split_and_unescape(&rsp_str);
        let args = args_vec_to_args(&args_vec);
        self.read_args(&args);
    }

    fn read_args<'a, I>(&mut self, args: I)
    where
        I: IntoIterator<Item = &'a ArgumentItem<MSVCArgAttribute>>,
    {
        for item in args.into_iter() {
            match item.data {
                Some(ResponseFile) => {
                    let path = item.arg.get_value().unwrap().unwrap_path();
                    let path_str = path.to_str().unwrap();
                    self.read_rsp_path(path_str);
                }
                None => match item.arg {
                    Argument::Raw(ref val) if val.is_empty() => {}
                    Argument::Raw(ref val) if !val.is_empty() => {
                        self.input_args.push(PathBuf::from(val))
                    }
                    Argument::UnknownFlag(ref flag) => self.unknown_flags.push(flag.clone()),
                    _ => unreachable!(),
                },
                Some(Output) => self.output_arg = item.arg.get_value().map(|v| v.unwrap_path()),
                Some(ProgramDatabase) => {
                    self.pdb_arg = item.arg.get_value().map(|v| v.unwrap_path())
                }
                Some(ref key) => {
                    self.args
                        .entry(key.clone())
                        .or_insert(Vec::new())
                        .push(item.arg.clone());
                }
            }
        }
    }
}

fn args_vec_to_args(args_vec: &[String]) -> Vec<MSVCArgItem> {
    let args_str_iter = args_vec
        .iter()
        .map(|ref arg| arg_normalize_prefix(arg))
        .map(OsString::from);
    ArgsIter::new(args_str_iter, &MSVC[..]).collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum MSVCArgAttribute {
    ResponseFile,
    Output,
    ProgramDatabase,
    DoCompilation,
    DebugInfo,
    PreprocessorArgument,
    ShowIncludes,
    DepFile,
    TooHard,
}
use self::MSVCArgAttribute::*;

static MSVC: [(ArgInfo, MSVCArgAttribute); 20] = [
    take_arg!("-D", String, CanBeSeparated, PreprocessorArgument),
    take_arg!("-FA", String, Concatenated, TooHard),
    take_arg!("-FI", Path, CanBeSeparated, PreprocessorArgument),
    take_arg!("-FR", Path, Concatenated, TooHard),
    take_arg!("-Fa", Path, Concatenated, TooHard),
    take_arg!("-Fd", Path, Concatenated, ProgramDatabase),
    take_arg!("-Fe", Path, Concatenated, TooHard),
    take_arg!("-Fi", Path, Concatenated, TooHard),
    take_arg!("-Fm", Path, Concatenated, TooHard),
    take_arg!("-Fo", Path, Concatenated, Output),
    take_arg!("-Fp", Path, Concatenated, TooHard),
    take_arg!("-Fr", Path, Concatenated, TooHard),
    flag!("-Fx", TooHard),
    take_arg!("-I", Path, Concatenated, PreprocessorArgument),
    take_arg!("-U", String, Concatenated, PreprocessorArgument),
    flag!("-Zi", DebugInfo),
    flag!("-c", DoCompilation),
    take_arg!("-deps", Path, Concatenated, DepFile),
    flag!("-showIncludes", ShowIncludes),
    take_arg!("@", Path, Concatenated, ResponseFile),
];

fn arg_normalize_prefix(arg: &str) -> String {
    if arg.starts_with("/") {
        let arg_str = arg.splitn(2, "/").last().unwrap();
        let dash = String::from("-");
        dash + arg_str
    } else {
        String::from(arg)
    }
}

fn rsp_file_read_into_string(rsp_path: &str) -> String {
    let mut rsp_file = File::open(rsp_path).expect("Can't open RSP file.");
    let mut rsp_content = Vec::new();
    rsp_file
        .read_to_end(&mut rsp_content)
        .expect("Can't read RSP file.");
    // .decode will choose the right decoder based on the BOM
    let (rsp_str, _, _) = UTF_16LE.decode(&rsp_content);
    return String::from(rsp_str);
}

fn string_split_and_unescape(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut arg = String::new();
    let mut chars = s.chars();
    let mut in_quotes = false;
    while let Some(ch) = chars.next() {
        match ch {
            '"' => in_quotes = !in_quotes,
            '\\' if in_quotes => match chars.next() {
                Some('\\') | None => arg.push('\\'),
                Some('"') => arg.push('"'),
                Some(ch2) => {
                    arg.push('\\');
                    arg.push(ch2);
                }
            },
            ' ' if !in_quotes => {
                args.push(arg);
                arg = String::new();
            }
            _ => arg.push(ch),
        }
    }
    if !arg.is_empty() {
        args.push(arg);
    }
    args
}

fn argument_to_os_string(argument: &Argument) -> Vec<OsString> {
    argument.clone().normalize(NormalizedDisposition::Concatenated).into_iter().collect()
}
