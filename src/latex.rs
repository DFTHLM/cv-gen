use crate::ast::*;

const PREAMBLE: &str = include_str!("latex/preamble.tex");

impl std::fmt::Display for CV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", PREAMBLE)?;
        writeln!(f, "\\input{{formatting.tex}}")?;
        writeln!(f, "{}", self.personal)?;
        Ok(())
    }
}

impl std::fmt::Display for Personal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.info)?;
        if let Some(edu) = &self.education {
            writeln!(f, "{}", edu)?;
        }
        if let Some(desc) = &self.desc {
            writeln!(f, "\\cvdesc{{{}}}", desc)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\\begin{{header}}")?;
        writeln!(f, "\\fontsize{{25 pt}}{{25 pt}}\\selectfont {}", self.name)?;
        writeln!(f, "\\vspace{{5 pt}}")?;
        writeln!(f, "\\normalise")?;
        writeln!(f, "\\kern 5.0 pt%")?;
        if let Some(email) = &self.email {
            writeln!(f, "\\mbox{{\\hrefWithoutArrow{{mailto:{}}}{{{}}}}}%",email, email)?;
            writeln!(f, "\\kern 5.0 pt%")?;
            writeln!(f, "\\AND%")?;
        }
        if let Some(tel) = &self.tel {
            writeln!(f, "\\cvcontact{{Phone Number: {}}}", tel)?;
            writeln!(f, "\\kern 5.0 pt%")?;
            writeln!(f, "\\AND%")?;
        }
        if let Some(github) = &self.github {
            writeln!(f, "\\cvcontact{{Github: {}}}", github)?;
            writeln!(f, "\\kern 5.0 pt%")?;
            writeln!(f, "\\AND%")?;
        }
        if let Some(linkedin) = &self.linkedin {
            writeln!(f, "\\cvcontact{{LinkedIn: {}}}", linkedin)?;
            writeln!(f, "\\kern 5.0 pt%")?;
            writeln!(f, "\\AND%")?;
        }
        if let Some(website) = &self.website {
            writeln!(f, "\\cvcontact{{Website: {}}}", website)?;
            writeln!(f, "\\kern 5.0 pt%")?;
            writeln!(f, "\\AND%")?;
        }
        writeln!(f, "\\end{{header}}")?;
        writeln!(f, "\\vspace{{5 pt - 0.3 cm}}")?;
        Ok(())
    }
}

impl std::fmt::Display for Education {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\\cvsection{{Education}}")?;
        for school in &self.school {
            writeln!(f, "{}", school)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for School {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\\cvitem{{{}}}{{{}}}", self.name, self.duration)?;
        if let Some(ending) = &self.ending {
            writeln!(f, "\\cvdesc{{{}}}", ending)?;
        }
        Ok(())
    }
}

