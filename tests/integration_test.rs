use common::{cleanup, server};
use pretty_assertions::assert_eq;
use reqwest::StatusCode;
use std::collections::HashMap;

mod common;

#[tokio::test(flavor = "multi_thread")]
async fn integration_test() {
    cleanup(true, None).await;
    let mut corolla = server("examples/example_spec.json").await;
    let inputs = ["sandringham", "beijing", "lombardy"];
    let client = reqwest::Client::new();
    for x in inputs.iter() {
        let mut body = HashMap::new();
        body.insert("vacation_spot", x);
        let res = client
            .post("http://localhost:50000/test/write/write01")
            .json(&body)
            .send()
            .await
            .expect("could not make HTTP request");
        assert_eq!(
            res.status(),
            StatusCode::OK,
            "HTTP request failed with message {:?}",
            res.text().await
        );
    }
    let res: Vec<Vec<String>> = reqwest::get("http://localhost:50000/test/read/read01")
        .await
        .expect("could not perform GET curl")
        .json()
        .await
        .expect("could not parse JSON into expected structure");
    for (i, row) in res.iter().enumerate() {
        assert_eq!(row.len(), 1);
        if i == 0 {
            assert_eq!(row.get(0).unwrap(), "vacation_spot");
        } else {
            let x = inputs.get(i - 1).unwrap();
            assert_eq!(row.get(0).unwrap(), x);
        }
    }
    cleanup(false, Some(&mut corolla)).await;
    let mut corolla = server("examples/example_spec_with_conversions.json").await;
    let inputs = [
        ("avon", "lovely"),
        ("seaside heights", "a fun town"),
        ("houston", "hot"),
    ];
    for (x, y) in inputs.iter() {
        let mut body = HashMap::new();
        body.insert("vacation_spot", x);
        body.insert("notes", y);
        let res = client
            .post("http://localhost:50000/test/write/write01")
            .json(&body)
            .send()
            .await
            .expect("could not make HTTP request");
        assert_eq!(
            res.status(),
            StatusCode::OK,
            "HTTP request failed with message {:?}",
            res.text().await
        );
    }
    let res: Vec<Vec<String>> = reqwest::get("http://localhost:50000/test/read/read01")
        .await
        .expect("could not perform GET curl")
        .json()
        .await
        .expect("could not parse JSON into expected structure");
    let mut iter = res.iter();
    assert_eq!(
        iter.next().unwrap(),
        &vec!["vacation_spot".to_string(), "notes".to_string()]
    );
    iter.next();
    iter.next();
    iter.next();
    for (i, row) in iter.enumerate() {
        assert_eq!(row.len(), 2);
        let (x, y) = inputs.get(i).unwrap();
        assert_eq!(row.get(0).unwrap(), x);
        assert_eq!(row.get(1).unwrap(), y);
    }
    cleanup(true, Some(&mut corolla)).await;
}
