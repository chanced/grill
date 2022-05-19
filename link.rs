use std::collections::BTreeMap;

use crate::hyper_schema::HyperSchema;
use crate::map::Map;
use crate::string::StringOrStrings;
use crate::value::Value;
// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Link {
    /// `anchor`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub anchor: Option<String>,
    /// `anchorPoint
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub anchor_point: Option<String>,
    /// `rel`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// - 04
    /// + `required`
    pub rel: Option<StringOrStrings>,
    /// `href`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// - 04
    /// + `required`
    pub href: Option<String>,
    /// `hrefSchema`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    ///+ default: `false`
    pub href_schema: Option<Box<HyperSchema>>,
    /// `templatePointers`
    /// - 2020-12
    /// - 2019-09
    pub template_pointers: Option<BTreeMap<String, String>>,
    /// `templateRequired`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + unique items
    pub template_required: Option<Vec<String>>,
    /// `title`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub title: Option<String>,
    /// `description`
    pub description: Option<String>,
    /// `targetSchema`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `true`
    pub target_schema: Option<HyperSchema>,
    /// `targetMediaType`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub target_media_type: Option<String>,
    /// `targetHints'
    pub target_hints: Option<Value>,
    /// `headerSchema`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `true`
    pub header_schema: Option<Box<HyperSchema>>,
    /// `submissionMediaType`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `"application/json"`
    pub submission_media_type: Option<String>,
    /// `submissionSchema`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `true`
    pub submission_schema: Option<HyperSchema>,

    /// `$comment`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub comment: Option<String>,

    // -------------------------------------------
    //                  draft 04
    // -------------------------------------------
    /// `template`
    /// - 04
    pub template: Option<String>,
    /// `method`
    /// - 04
    /// + default: `"GET"`
    pub method: Option<String>,

    // -------------------------------------------
    //           additional_fields
    // -------------------------------------------
    pub additional_fields: Map<String, Value>,
}
