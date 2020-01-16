use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum RuleType {
    HasArg,
    NoArg,
    Maybe,
}

#[derive(Clone, Copy)]
struct Rule<'a> {
    rule_type: RuleType,
    default_value: Option<&'a str>,
}

pub struct CliParser<'a, 'b> {
    rules: HashMap<&'a str, Rule<'b>>,
}

#[derive(Clone, Copy)]
struct Opt<'a> {
    argument: Option<&'a str>,
    rule_type: RuleType,
}

pub struct Matches<'a, 'b> {
    opts: HashMap<&'a str, Opt<'b>>,
}

impl<'a, 'b> CliParser<'a, 'b> {
    pub fn new() -> Self {
        CliParser {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule_with_default(
        &mut self,
        name: &'a str,
        rule_type: RuleType,
        default_value: &'b str,
    ) -> Result<(), &'static str> {
        match rule_type {
            RuleType::Maybe => {
                self.rules.insert(
                    name,
                    Rule {
                        rule_type: RuleType::Maybe,
                        default_value: Some(default_value),
                    },
                );
                Ok(())
            }
            RuleType::NoArg => {
                self.rules.insert(
                    name,
                    Rule {
                        rule_type: RuleType::NoArg,
                        default_value: Some(default_value),
                    },
                );
                Ok(())
            }
            RuleType::HasArg => Err("Default value is meaningless!"),
        }
    }

    pub fn add_rule(&mut self, name: &'a str) -> Result<&'static str, ()> {
        self.rules.insert(
            name,
            Rule {
                rule_type: RuleType::HasArg,
                default_value: None,
            },
        );
        Ok("Rule added")
    }

    pub fn parse(&self, arg_list: &'a Vec<String>) -> Result<Matches, &'static str> {
        let mut opts: HashMap<&str, Opt> = HashMap::new();

        if arg_list.len() < 2 {
            return Err("No opt to parse!");
        } else {
            let mut index: usize = 1;
            while index < arg_list.len() {
                if arg_list[index].chars().nth(0).unwrap() != '+' {
                    return Err("The input contains no flag");
                }

                match self.rules.get(&arg_list[index][1..]) {
                    Some(rule) => {
                        match rule.rule_type {
                            RuleType::NoArg => {
                                opts.insert(
                                    &arg_list[index][1..],
                                    Opt {
                                        argument: None,
                                        rule_type: RuleType::NoArg,
                                    },
                                );
                            }
                            RuleType::HasArg => {
                                if index + 1 >= arg_list.len() {
                                    return Err("Need argument: Index out of bound");
                                }
                                if arg_list[index + 1].chars().nth(0).unwrap() == '+' {
                                    return Err("Need argument!");
                                }

                                opts.insert(
                                    &arg_list[index][1..],
                                    Opt {
                                        argument: Some(&arg_list[index + 1]),
                                        rule_type: rule.rule_type,
                                    },
                                );
                                index += 1; // Eat it
                            }
                            RuleType::Maybe => {
                                if index + 1 >= arg_list.len()
                                    || arg_list[index + 1].chars().nth(0).unwrap() == '+'
                                {
                                    opts.insert(
                                        &arg_list[index][1..],
                                        Opt {
                                            argument: None,
                                            rule_type: RuleType::Maybe,
                                        },
                                    );
                                } else {
                                    opts.insert(
                                        &arg_list[index][1..],
                                        Opt {
                                            argument: Some(&arg_list[index + 1]),
                                            rule_type: RuleType::Maybe,
                                        },
                                    );
                                    index += 1; // Eat it
                                }
                            }
                        };
                    }
                    _ => return Err("Cannot recognize rule"),
                };
                index += 1;
            }
        }

        Ok(Matches { opts: opts })
    }
}

impl<'a, 'b> Matches<'a, 'b> {
    pub fn get_opt(&self, opt_str: &'b str) -> Result<Option<&'b str>, &'a str> {
        match self.opts.get(opt_str) {
            Some(target_opt) => match target_opt.rule_type {
                RuleType::NoArg => Ok(None),
                _ => Ok(target_opt.argument),
            },
            _ => Err("No argument found"),
        }
    }
}
