pub mod argparse {
    pub struct Argument {
        pub trigger: &'static str,
        pub mtrigger: &'static str,
        pub isflag: bool,
        pub isrequired: bool,
    }

    pub struct ArgumentParser { 
        pub argsvec: Vec<String>,
        pub prefix: char,
        pub args: &'static [Argument]
    }

    impl ArgumentParser {
        pub fn compile(&self) -> FinalizedArguments {
            let mut fargs: Vec<FinalizedArgument> = vec![];
            let mut reading: bool = false;
            let mut current: &'static Argument = &Argument { trigger: "", mtrigger: "", isflag: false, isrequired: false };
            let mut current_val: String = "".to_string();
            let mut i: usize = 0;

            let mut _trigs: Vec<String> = vec![];
            let mut _mtrigs: Vec<String> = vec![];
            for arg in self.args {
                if _trigs.contains(&arg.trigger.to_string()) { panic!("{}", format!("FATAL redefinition of '{}' / '{}'", arg.trigger, arg.mtrigger)) }
                if _mtrigs.contains(&arg.mtrigger.to_string()) { panic!("{}", format!("FATAL redefinition of '{}' / '{}'", arg.trigger, arg.mtrigger)) }
                _trigs.push(arg.trigger.to_string());
                _mtrigs.push(arg.mtrigger.to_string());
            }

            for a in self.get_args() {
                i += 1;
                let aspl: Vec<_> = a.split(self.prefix).collect();
                let astr: &str = aspl[aspl.len()-1];
                if reading {
                    if reading && a.starts_with(self.prefix) { 
                        if current_val.is_empty() { panic!("{}", format!("cannot define new variable ('{}') before defining the variable before ('{}')", a, current.trigger)); }
                        fargs.push(FinalizedArgument { arg: current, val: current_val.clone().trim().to_string(), flagged: false });
                        current = &Argument { trigger: "", mtrigger: "", isflag: false, isrequired: false };
                        if current_val.is_empty() { panic!("FATAL attempting to add null value for '{}'", a); }

                        current_val = "".to_string();
                        reading = false;

                        let mut found = false;
                        for arg in self.args {
                            if (arg.trigger == astr && aspl.len() == 3) || (arg.mtrigger == astr && aspl.len() == 2) {
                                current = arg;
                                found = true;

                                if arg.isflag { fargs.push(FinalizedArgument { arg: current, val: "".to_string(), flagged: true }); } 
                                else { reading = true; }
                                break;
                            }
                        }
                        if !found { panic!("FATAL trigger '{}' not found", a); }
                    } else { current_val = format!("{}{} ", current_val, astr);}
                } else {
                    if !a.starts_with(self.prefix) { continue; }
                    let mut found = false;
                    for arg in self.args {
                        if (arg.trigger == astr && aspl.len() == 3) || (arg.mtrigger == astr && aspl.len() == 2) {
                            current = arg;
                            found = true;
                            
                            if arg.isflag { fargs.push(FinalizedArgument { arg: current, val: "".to_string(), flagged: true }); } 
                            else { reading = true; }
                            break;
                        }
                    }
                    if !found { panic!("FATAL trigger '{}' not found", a); }
                }

                if i == self.get_args().len() { 
                    if current_val.is_empty() && !current.isflag { panic!("attempting to add null value for '{}'", a); }
                    else if !current.isflag { fargs.push(FinalizedArgument { arg: current, val: current_val.clone().trim().to_string(), flagged: false }); }
                }
            }

            let mut _utrigs: Vec<String> = vec![];
            let mut _umtrigs: Vec<String> = vec![];
            for arg in fargs.iter().clone() {
                if _utrigs.contains(&arg.arg.trigger.to_string()) { panic!("{}", format!("FATAL redefinition of '{}' / '{}' in command line", arg.arg.trigger, arg.arg.mtrigger)) }
                if _umtrigs.contains(&arg.arg.mtrigger.to_string()) { panic!("{}", format!("FATAL redefinition of '{}' / '{}' in command line", arg.arg.trigger, arg.arg.mtrigger)) }
                _utrigs.push(arg.arg.trigger.to_string());
                _umtrigs.push(arg.arg.mtrigger.to_string());
            }

            let mut missing: Vec<&str> = vec![];
            for a in self.args {
                let mut found = false;
                for fa in fargs.iter().clone() {
                    if fa.arg.trigger == a.trigger && fa.arg.mtrigger == a.mtrigger { 
                        found = true;
                        break;
                    }
                }

                if !found {
                    if a.isflag { fargs.push(FinalizedArgument { arg: a, val: "".to_string(), flagged: false }); }
                    if !a.isrequired { continue; }
                    missing.push(a.trigger);
                }
            }

            if missing.len() > 0 { panic!("FATAL missing required argument(s): {}", missing.join(", ")) }
            return FinalizedArguments { args: fargs }
        }

        fn get_args(&self) -> Vec<String> {
            let mut l = self.argsvec.clone();
            l.remove(0);
            return l;
        }
    }

    pub struct FinalizedArgument {
        pub arg: &'static Argument,
        pub val: String,
        pub flagged: bool
    }

    pub struct FinalizedArguments {
        pub args: Vec<FinalizedArgument>
    }

    impl FinalizedArguments {
        pub fn get(&self, trig: &str) -> Option<&FinalizedArgument> {
            for a in self.args.iter().clone() {
                if a.arg.trigger == trig { return Some(a); }
            }
            
            return None;
        }
    }
}