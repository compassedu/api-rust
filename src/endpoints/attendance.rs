use serde::{Deserialize, Serialize};

use crate::utils::consts::USER_AGENT;

pub async fn get_attendance_summary(
    cookies: String,
    start_date: String,
    end_date: String,
    user_id: i32,
    school_id: &str,
) -> Result<Vec<AttendanceSummary>, reqwest::Error> {
    let url = format!(
        "https://{}.compass.education/Services/Attendance.svc/GetAttendanceSummary",
        school_id
    );
    let mut headers = reqwest::header::HeaderMap::new();
    let cookies_str: &str = cookies.as_str();
    headers.insert(
        "cookie",
        reqwest::header::HeaderValue::from_str(&cookies_str).unwrap(),
    );
    let req = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .default_headers(headers)
        .user_agent(USER_AGENT)
        .build()?
        .post(url)
        .json(&GetAttendanceSumReq {
            start_date,
            end_date,
            student_status: "2".to_string(),
            in_class: ["0".to_string(), "1".to_string()].to_vec(),
            ok_class: ["0".to_string(), "1".to_string()].to_vec(),
            vce: ["0".to_string(), "1".to_string()].to_vec(),
            schl: ["0".to_string(), "1".to_string()].to_vec(),
            perspective: "1".to_string(),
            total_whole_day_limit: "0".to_string(),
            total_partial_day_limit: "0".to_string(),
            user_id: user_id.to_string(),
        })
        .send()
        .await?;
    let res = req.json::<GetAttendanceSumRes>().await?;
    let mut attendance_summary: Vec<AttendanceSummary> = Vec::new();
    for subject in res.d {
        let v: AttendanceSummary = AttendanceSummary {
            authorized_excused_absence: subject.authorized_excused_absence,
            attendance_id: subject.attendance_id,
            attendance_name: subject.attendance_name,
            authorized_absence: subject.authorized_absence,
            count: subject.count,
            counted: subject.counted,
            form: subject.form,
            total_out_of_class: subject.total_out_of_class,
            late_unexplained: subject.late_unexplained,
            last_name: subject.last_name,
            lu: subject.lu,
            not_present_counted_vce_ok: subject.not_present_counted_vce_ok,
            notcounted: subject.notcounted,
            npa: subject.npa,
            npu: subject.npu,
            p: subject.p,
            pa: subject.pa,
            pok: subject.pok,
            subject_name: subject.subject_name,
            spc: subject.spc,
            ta: subject.ta,
            tna: subject.tna,
            user_id: subject.user_id,
            un: subject.un,
            unauthorizedabsence: subject.unauthorizedabsence,
            vceok: subject.vceok,
            vpc: subject.vpc,
        };
        attendance_summary.push(v);
    }
    Ok(attendance_summary)
}
#[derive(Serialize, Deserialize, Debug)]
struct GetAttendanceSumReq {
    #[serde(rename = "startDate")]
    start_date: String,
    #[serde(rename = "endDate")]
    end_date: String,
    #[serde(rename = "studentStatus")]
    student_status: String,
    #[serde(rename = "inClass")]
    in_class: Vec<String>,
    #[serde(rename = "okClass")]
    ok_class: Vec<String>,
    vce: Vec<String>,
    schl: Vec<String>,
    perspective: String,
    #[serde(rename = "totalWholeDayLimit")]
    total_whole_day_limit: String,
    #[serde(rename = "totalPartialDayLimit")]
    total_partial_day_limit: String,
    #[serde(rename = "userId")]
    user_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct AttendanceSumRes {
    #[serde(rename = "aea")]
    authorized_excused_absence: i32,
    #[serde(rename = "aid")]
    attendance_id: i32,
    #[serde(rename = "an")]
    attendance_name: String,
    #[serde(rename = "authorizedabsence")]
    authorized_absence: i32,
    #[serde(rename = "c")]
    count: i32,
    #[serde(rename = "counted")]
    counted: i32,
    #[serde(rename = "fg")]
    form: String,
    #[serde(rename = "im")]
    total_out_of_class: i32,
    #[serde(rename = "la")]
    late_unexplained: i32,
    #[serde(rename = "ln")]
    last_name: String,
    // #[serde(rename = "lu")]
    lu: i32,
    #[serde(rename = "notPresentCountedVceOk")]
    not_present_counted_vce_ok: i32,
    // #[serde(rename = "ln")]
    notcounted: i32,
    // #[serde(rename = "ln")]
    npa: i32,
    // #[serde(rename = "ln")]
    npu: i32,
    // #[serde(rename = "ln")]
    p: i32,
    // #[serde(rename = "ln")]
    pa: i32,
    // #[serde(rename = "ln")]
    pok: String,
    #[serde(rename = "sn")]
    subject_name: String,
    // #[serde(rename = "ln")]
    spc: String,
    // #[serde(rename = "ln")]
    ta: i32,
    // #[serde(rename = "ln")]
    tna: i32,
    // #[serde(rename = "uid")]
    user_id: i32,
    // #[serde(rename = "ln")]
    un: String,
    // #[serde(rename = "ln")]
    unauthorizedabsence: i32,
    // #[serde(rename = "ln")]
    vceok: i32,
    // #[serde(rename = "ln")]
    vpc: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AttendanceSummary {
    pub authorized_excused_absence: i32,
    pub attendance_id: i32,
    pub attendance_name: String,
    pub authorized_absence: i32,
    pub count: i32,
    pub counted: i32,
    pub form: String,
    pub total_out_of_class: i32,
    pub late_unexplained: i32,
    pub last_name: String,
    pub lu: i32,
    pub not_present_counted_vce_ok: i32,
    pub notcounted: i32,
    pub npa: i32,
    pub npu: i32,
    pub p: i32,
    pub pa: i32,
    pub pok: String,
    pub subject_name: String,
    pub spc: String,
    pub ta: i32,
    pub tna: i32,
    pub user_id: i32,
    pub un: String,
    pub unauthorizedabsence: i32,
    pub vceok: i32,
    pub vpc: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct GetAttendanceSumRes {
    d: Vec<AttendanceSumRes>,
}
// pub async fn get_periods_for_timeline(
//     cookies: String,
//     date: String,
//     user_id: i32,
//     school_id: &str,
// ) {
// }
// pub async fn get_half_day_summary_grid_lines(
//     cookies: String,
//     year: i32,
//     user_id: i32,
//     school_id: &str,
// ) {
// }
