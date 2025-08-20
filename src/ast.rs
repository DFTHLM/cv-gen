#[derive(Debug)]
pub struct CV {
    pub personal: Personal,
    pub main: Main,
}

// Personal section

#[derive(Debug)]
pub struct Personal {
    pub info: Info,
    pub education: Option<Education>,
    pub desc: Option<String>,
}

#[derive(Debug)]
pub struct Info {
    pub name: String,
    pub email: Option<String>,
    pub tel: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug)]
pub struct School {
    pub name: String,
    pub ending: Option<String>,
    pub duration: String,
}

#[derive(Debug)]
pub struct Education {
    pub school: Vec<School>,
}

// Main section

#[derive(Debug)]
pub struct Main {
    pub experience: Option<Vec<Job>>,
    pub skills: Option<Skills>,
    pub projects: Option<Vec<Project>>,
    pub awards: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Job {
    pub title: String,
    pub company: String,
    pub duration: Option<String>,
    pub desc: Option<String>,
}

#[derive(Debug)]
pub struct Skills {
    pub highlighted: Option<Vec<Skill>>,
    pub list: Option<String>
}

#[derive(Debug)]
pub struct Skill {
    pub name: String,
    pub level: String,
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub desc: Option<String>,
    pub tech: Option<String>,
}

