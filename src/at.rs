use regex::RegexBuilder;
use std::str;

struct ATParser {
    data: String,
    regex_echo: regex::Regex,
    regex_no_echo: regex::Regex,
    at_command: String,
    result: String,
    return_code: String,
}

// regex_rc: RegexBuilder::new(r"^(OK|CONNECT|RING|NO CARRIER|ERROR|NO DIALTONE|BUSY|NO ANSWER|ERROR1|.*ONGOING.*)[\r\n]$").multi_line(true).build().unwrap(),
// regex_res_echo: RegexBuilder::new(r"(?P<at_command>[^\r\n]+)[\r\n]+((?P<result>[^\r\n]+)[\r\n]+)?(?P<return_code>[^\r\n]+)[\r\n]+").multi_line(true).build().unwrap(),

// (?P<at>AT.*[\r\n])(?P<res>(.*[\r\n]){1,})(?P<rc>OK|CONNECT|RING|NO CARRIER|ERROR|NO DIALTONE|BUSY|NO ANSWER|ERROR1|.*ONGOING.*)[\r\n]
//             regex_test: RegexBuilder::new(r"(?P<at_command>AT.*)\n+(?P<result>(.*[\r\n]){1,})(?P<return_code>OK|CONNECT|RING|NO CARRIER|ERROR|NO DIALTONE|BUSY|NO ANSWER|ERROR1|.*ONGOING.*)[\r\n]").multi_line(true).build().unwrap(),


impl ATParser {
    pub fn new() -> Self {
        ATParser {
            data: String::with_capacity(1024),
            regex_echo: RegexBuilder::new(r"(?P<at_command>AT.*)\n+(?P<result>(.*\n){0,})(?P<return_code>OK|CONNECT|RING|NO CARRIER|ERROR|NO DIALTONE|BUSY|NO ANSWER|ERROR1|.*ONGOING.*)\n").multi_line(true).build().unwrap(),
            regex_no_echo: RegexBuilder::new(r"(?P<result>(.*\n){0,})(?P<return_code>OK|CONNECT|RING|NO CARRIER|ERROR|NO DIALTONE|BUSY|NO ANSWER|ERROR1|.*ONGOING.*)\n").multi_line(true).build().unwrap(),
            at_command: String::new(),
            result: String::new(),
            return_code: String::new(),
        }
    }

    pub fn process(&mut self, data: &str) -> Option<regex::Captures> {
        self.data.push_str(&Self::convert_cr_lf(data));

        if self.regex_echo.is_match(&self.data) {
            if let Some(caps) = self.regex_echo.captures(&self.data) {
                println!("captured echo {:?}", caps);

                if let Some(at_cmd) = caps.name("at_command") {
                    self.at_command = at_cmd.as_str().to_string();
                }

                if let Some(result) = caps.name("result") {
                    self.result = result.as_str().to_string();
                    if self.result.ends_with("\n") {
                        self.result = self.result[..self.result.len() - 1].to_string();
                    }
                }

                if let Some(rc) = caps.name("return_code") {
                    self.return_code = rc.as_str().to_string();
                }

                return Some(caps);
            }
        }
        else  if self.regex_no_echo.is_match(&self.data) {
            if let Some(caps) = self.regex_no_echo.captures(&self.data) {
                println!("captured no echo {:?}", caps);

                if let Some(x) = caps.name("result") {
                    self.result = x.as_str().to_string().replace("\n", "");
                }

                if let Some(x) = caps.name("return_code") {
                    self.return_code = x.as_str().to_string();
                }

                return Some(caps);
            }
        }
        else {
            println!("no match");
        }
        None
    }

    pub fn at_command(&self) -> &String {
        &self.at_command
    }

    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn return_code(&self) -> &String {
        &self.return_code
    }

    pub fn convert_cr_lf(value: &str) -> String {
        let data_without_cr = value.replace("\r", "\n");
        data_without_cr.to_string().replace("\n\n", "\n")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_response() {
        let mut dut = ATParser::new();
        let res = dut.process("");
        assert!(res.is_none());
        assert_eq!(dut.return_code(), "");
        assert_eq!(dut.result(), "");
        assert_eq!(dut.at_command(), "");
    }

    #[test]
    fn empty_line() {
        let mut dut = ATParser::new();
        let res = dut.process("\n\r");
        assert!(res.is_none());
        assert_eq!(dut.return_code(), "");
        assert_eq!(dut.result(), "");

        let mut dut = ATParser::new();
        let res = dut.process("\n");
        assert!(res.is_none());
        assert_eq!(dut.return_code(), "");
        assert_eq!(dut.result(), "");
    }

    #[test]
    fn line_endings() {
        let mut dut = ATParser::new();
        let res = dut.process("ATI\nTOBY-L210-03S-01\nOK\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");

        let mut dut = ATParser::new();
        let res = dut.process("ATI\rTOBY-L210-03S-01\rOK\r");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");

        let mut dut = ATParser::new();
        let res = dut.process("ATI\r\nTOBY-L210-03S-01\r\nOK\r\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");

        let mut dut = ATParser::new();
        let res = dut.process("ATI\n\rTOBY-L210-03S-01\n\rOK\n\r");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");
    }

    #[test]
    fn ati() {
        let mut dut = ATParser::new();
        let res = dut.process("ATI\r\nTOBY-L210-03S-01\r\nOK\r\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");
        assert_eq!(dut.at_command(), "ATI");
        assert_eq!(dut.result(), "TOBY-L210-03S-01");
    }

    #[test]
    fn ati_line_by_line() {
        let mut dut = ATParser::new();
        let res = dut.process("ATI\r\n");
        assert!(res.is_none());
        let res = dut.process("TOBY-L210-03S-01\r\n");
        assert!(res.is_none());
        let res = dut.process("OK\r\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");
        assert_eq!(dut.at_command(), "ATI");
        assert_eq!(dut.result(), "TOBY-L210-03S-01");
    }

    #[test]
    fn at_plus_ccid() {
        let mut dut = ATParser::new();
        let res = dut.process("AT+CCID\r\n+CCID: 89882390000056327672\r\nOK\r\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");
        assert_eq!(dut.at_command(), "AT+CCID");
        assert_eq!(dut.result(), "+CCID: 89882390000056327672");
    }

    #[test]
    fn at_plus_cfun() {
        let mut dut = ATParser::new();
        let res = dut.process("AT+CFUN?\r\n+CFUN: 1,0\r\nOK\r\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");
        assert_eq!(dut.at_command(), "AT+CFUN?");
        assert_eq!(dut.result(), "+CFUN: 1,0");
    }

    #[test]
    fn at_plus_cgmi() {
        let mut dut = ATParser::new();
        let res = dut.process("AT+CGMI\r\nu-blox\r\nOK\r\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");
        assert_eq!(dut.at_command(), "AT+CGMI");
        assert_eq!(dut.result(), "u-blox");
    }


    #[test]
    fn at_plus_cnum_no_results() {
        let mut dut = ATParser::new();
        let res = dut.process("AT+CNUM\nOK\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");
        assert_eq!(dut.at_command(), "AT+CNUM");
    }

    #[test]
    fn unknown_at_cmd_error() {
        let mut dut = ATParser::new();
        let res = dut.process("ATunknown\r\nERROR\r\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "ERROR");
        assert_eq!(dut.result(), "");
    }

    /* TODO: not working as expected */
    #[test]
    fn multiple_results() {
        let mut dut = ATParser::new();
        let res = dut.process("ATX\r\nLine1\r\nLine2\r\nOK\r\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");
        assert_eq!(dut.result(), "Line1\nLine2");
        // TODO: Test result
    }

    #[test]
    fn ate0_echo_off() {
        let mut dut = ATParser::new();
        let res = dut.process("ATE0\nOK\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");
        assert_eq!(dut.at_command(), "ATE0");
    }

    #[test]
    fn ati9_echo_off() {
        let mut dut = ATParser::new();
        let res = dut.process("16.19,A01.04\nOK\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");
        assert_eq!(dut.result(), "16.19,A01.04");
    }

    #[test]
    fn echo_off_no_result() {
        let mut dut = ATParser::new();
        let res = dut.process("OK\n");
        assert!(res.is_some());
        assert_eq!(dut.return_code(), "OK");
    }
}


// Unsolicited
//  - can happen any time until AT command is sent (newline character)
//    from then on response is guaranteed not to be interrupted
// +/- echo
//
