use anyhow::anyhow;
use miniserde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CourseContent {
    pub sections: Vec<CourseSection>,
}

#[derive(Debug, Deserialize)]
pub struct CourseSection {
    pub header: String,
    pub text: Option<String>,
    pub list: Vec<CourseItemRaw>,
}

#[derive(Debug)]
pub enum CourseItem<'a> {
    Text {
        name: &'a str,
        id: u64,
        link: &'a str,
    },
    Link {
        name: &'a str,
        link: &'a str,
    },
    Task {
        name: &'a str,
        id: u64,
        link: &'a str,
        status: CourseTaskStatus,
    },
}

#[derive(Debug, Deserialize)]
pub struct CourseItemRaw {
    #[serde(rename = "objectType")]
    object_type: CourseItemType,
    name: String,
    id: Option<String>,
    link: String,
    status: Option<CourseTaskStatus>,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum CourseTaskStatus {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fail")]
    Fail,
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Deserialize)]
pub enum CourseItemType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "task")]
    Task,
}

impl CourseItemRaw {
    pub fn as_enum(&self) -> anyhow::Result<CourseItem<'_>> {
        Ok(match &self.object_type {
            CourseItemType::Text => CourseItem::Text {
                name: &self.name,
                id: self
                    .id
                    .clone()
                    .ok_or_else(|| anyhow!("Could not get ID"))?
                    .parse()?,
                link: &self.link,
            },
            CourseItemType::Link => CourseItem::Link {
                name: &self.name,
                link: &self.link,
            },
            CourseItemType::Task => CourseItem::Task {
                name: &self.name,
                id: self
                    .id
                    .clone()
                    .ok_or_else(|| anyhow!("Could not get ID"))?
                    .parse()?,
                link: &self.link,
                status: self.status.ok_or_else(|| anyhow!("Could not get status"))?,
            },
        })
    }
}
