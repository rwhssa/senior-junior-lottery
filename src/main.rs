use csv::ReaderBuilder;
use csv::WriterBuilder;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Clone)]
struct Student {
    class: String,
    number: u32,
    name: String,
    contact_method: Option<String>,
    contact_info: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // 讀取 CSV 檔案
    let file1 = File::open("file1.csv")?;
    let file2 = File::open("file2.csv")?;

    let mut rdr1 = ReaderBuilder::new().from_reader(file1);
    let mut rdr2 = ReaderBuilder::new().from_reader(file2);

    let students1: Vec<Student> = rdr1.deserialize().collect::<Result<_, _>>()?;
    let students2: Vec<Student> = rdr2.deserialize().collect::<Result<_, _>>()?;

    // 根據班級分組
    let mut group1: HashMap<String, Vec<Student>> = HashMap::new();
    let mut group2: HashMap<String, Vec<Student>> = HashMap::new();

    for student in students1 {
        group1.entry(student.class.clone()).or_default().push(student);
    }
    
    for student in students2 {
        group2.entry(student.class.clone()).or_default().push(student);
    }

    let mut final_results = Vec::new();
    let mut unmatched_students1 = Vec::new();
    let mut unmatched_students2 = Vec::new();

    // 處理每對班級的配對
    for (class1, mut students1) in group1 {
        // 假設高一的班級與高二班級的班級代號相差 100
        let corresponding_class = format!("5{}", &class1[1..]);  // e.g., 401 -> 501
        if let Some(mut students2) = group2.remove(&corresponding_class) {
            // 將學生隨機打亂
            students1.shuffle(&mut rand::thread_rng());
            students2.shuffle(&mut rand::thread_rng());

            let len = std::cmp::min(students1.len(), students2.len());

            // 配對學生
            for i in 0..len {
                final_results.push((students1[i].clone(), students2[i].clone()));
            }

            // 將無法配對的學生加入未配對列表
            if students1.len() > len {
                unmatched_students1.extend(students1[len..].to_vec());
            } else if students2.len() > len {
                unmatched_students2.extend(students2[len..].to_vec());
            }
        } else {
            unmatched_students1.extend(students1);
        }
    }

    // 將無法配對的學生加入到未配對列表
    for (_class, students2) in group2 {
        unmatched_students2.extend(students2);
    }

    // 處理剩餘的高一和高二未配對學生
    unmatched_students1.shuffle(&mut rand::thread_rng());
    unmatched_students2.shuffle(&mut rand::thread_rng());

    let len_unmatched = std::cmp::min(unmatched_students1.len(), unmatched_students2.len());

    for i in 0..len_unmatched {
        final_results.push((unmatched_students1[i].clone(), unmatched_students2[i].clone()));
    }

    // 將無法配對的剩餘學生處理
    if unmatched_students1.len() > len_unmatched {
        unmatched_students1 = unmatched_students1[len_unmatched..].to_vec();
    } else {
        unmatched_students2 = unmatched_students2[len_unmatched..].to_vec();
    }

    // 輸出到 CSV
    let output_file = File::create("final_result.csv")?;
    let mut wtr = WriterBuilder::new().from_writer(output_file);

    wtr.write_record(&["班級1", "座號1", "姓名1", "聯絡管道1", "聯絡資訊1", 
                       "班級2", "座號2", "姓名2", "聯絡管道2", "聯絡資訊2"])?;

    for (student1, student2) in final_results {
        let record1 = vec![
            student1.class.clone(),
            student1.number.to_string(),
            student1.name.clone(),
            student1.contact_method.clone().unwrap_or_else(|| "".to_string()),
            student1.contact_info.clone().unwrap_or_else(|| "".to_string()),
        ];

        let record2 = vec![
            student2.class.clone(),
            student2.number.to_string(),
            student2.name.clone(),
            student2.contact_method.clone().unwrap_or_else(|| "".to_string()),
            student2.contact_info.clone().unwrap_or_else(|| "".to_string()),
        ];

        wtr.write_record(record1.into_iter().chain(record2.into_iter()))?;
    }


    wtr.flush()?;
    Ok(())
}
