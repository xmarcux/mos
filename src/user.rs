#[derive(Debug)]
pub enum Available {
    Available,
    Busy,
    VideoConference,
    OnThePhone,
    OutOfOffice,
    DoNotDesturb,
}

#[derive(Debug)]
pub struct User {
    pub nickname: String,
    pub name: String,
    pub surname: String,
    pub profession: String,
    pub available: Available,
}

impl User {
    /// Function returns formated string to write to file
    pub fn to_frm_str(self) -> String {
        let mut s_avail: &str;
        let mut s: String = format!("nickname={}", self.nickname);
        s = format!("{}\nname={}", s, self.name);
        s = format!("{}\nsurname={}", s, self.surname);
        s = format!("{}\nprofession={}", s, self.profession);

        s_avail = match self.available {
            Available::Available       => "Available",
            Available::Busy            => "Busy",
            Available::VideoConference => "VideoConference",
            Available::OnThePhone      => "OnThePhone",
            Available::OutOfOffice     => "OutOfOffice",
            Available::DoNotDesturb    => "DoNotDesturb",
        };
        
        s = format!("{}\navailable={}\n", s, s_avail);
        s
    }

    /// Creates a new User from formated string
    pub fn from_frm_str(frm_str: String) -> User {
        let frm_v: Vec<&str> = frm_str.split("\n").collect();
        let mut nick: String = String::new();
        let mut name: String = String::new();
        let mut sur: String = String::new();
        let mut prof: String = String::new();
        let mut avail: Available = Available::Available;
        
        for l in frm_v {
            let l_t = l.trim();
            let l_s: Vec<&str> = l_t.split("=").collect();

            if l_s.len() >= 2 {
                if l_t.starts_with("nickname=") {
                    nick = l_s[1].to_string();
                }
                else if l_t.starts_with("name=") {
                    name = l_s[1].to_string();
                }
                else if l_t.starts_with("surname=") {
                    sur = l_s[1].to_string();
                }
                else if l_t.starts_with("profession=") {
                    prof = l_s[1].to_string();
                }
                else if l_t.starts_with("available=") {
                    avail = match l_s[1] {
                        "Busy"            => Available::Busy,
                        "VideoConference" => Available::VideoConference,
                        "OnThePhone"      => Available::OnThePhone,
                        "OutOfOffice"     => Available::OutOfOffice,
                        "DoNotDesturb"    => Available::DoNotDesturb,
                        _                 => Available::Available,
                    };
                }

            }
        }

        User { nickname: nick, name: name, surname: sur, profession: prof, available: avail }
    }
}
