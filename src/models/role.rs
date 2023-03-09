use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Role {
    Admin = 1,
    Moderator = 2,
    User = 3,
}

impl Role {
    pub fn from_u8(n: u8) -> Result<Role, String> {
        match n {
            1 => Ok(Role::Admin),
            2 => Ok(Role::Moderator),
            3 => Ok(Role::User),
            _ => Err("Expected a number from: 1, 2 or 3".to_owned()),
        }
    }

    pub fn to_u8(role: Role) -> u8 {
        match role {
            Role::Admin => 1,
            Role::Moderator => 2,
            Role::User => 3,
        }
    }
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Admin => "admin".to_owned(),
            Role::Moderator => "moderator".to_owned(),
            Role::User => "user".to_owned(),
        }
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "admin" => Ok(Role::Admin),
            "moderator" => Ok(Role::Moderator),
            "user" => Ok(Role::User),
            _ => Err(()),
        }
    }
}
