# Main before loading the window:
```rust
    /*
    checksum for "resources\\FVDPA5hUEAApvwP.jpg" :: "7a95a2b5c3b5ae743c233c138307e3159a44f624fdcda0eec641b37b1859bc80"
    checksum for "resources\\FVNm8h8VUAAPqJR.jpg" :: "e2c5626baca673c40b41f6a2673bafd3ca02005e8bf21b7d1d78de2ee8690066"
    checksum for "resources\\FVP2SqSaQAAqiWu.jpg" :: "fc821528d427c906ad1be41e1c5350657ff6bd3f8e60758550cf202161d593f8"
    checksum for "resources\\FXm37GQaMAAAYs7.png" :: "066aa787610eb2eacd2ca207947afd524ae128f02a11c767ec93ac2c5ada3fb1"
    checksum for "resources\\qFVIAKUKaMAASwp6.jpg" :: "64bf3282c8b9c8ab81fbf581f97113348fdaa87ae0ba8765bab382a93e87492b"
	*/
    let contacts: HashMap<_, _> = phones
        .into_iter()
        .map(|(key, phone)| {
            (
                key,
                Contact {
                    phone,
                    address: addresses.remove(key).unwrap(),
                },
            )
        })
        .collect();
         xs.retain(|&x| x != some_x);

    // Why would i do it this way? Isn't it much more logical to make tags keys and checksums as values
    //let mut tags = HashMap::new();
    images.tags.insert(String::from("7a95a2b5c3b5ae743c233c138307e3159a44f624fdcda0eec641b37b1859bc80"), vec![String::from("test_tag_1"), String::from("blue")]);
    images.tags.insert(String::from("e2c5626baca673c40b41f6a2673bafd3ca02005e8bf21b7d1d78de2ee8690066"), vec![String::from("test_tag_2"), String::from("red"), String::from("blue")]);
    images.tags.insert(String::from("fc821528d427c906ad1be41e1c5350657ff6bd3f8e60758550cf202161d593f8"), vec![String::from("test_tag_1"), String::from("red")]);
    images.tags.insert(String::from("066aa787610eb2eacd2ca207947afd524ae128f02a11c767ec93ac2c5ada3fb1"), vec![String::from("red")]);
    images.tags.insert(String::from("64bf3282c8b9c8ab81fbf581f97113348fdaa87ae0ba8765bab382a93e87492b"), vec![String::from("yellow")]);

    let mut test_hm: Vec<String> = images.tags.clone()
        .into_iter()
        .map(|(key, value)| {
            if value.contains(&String::from("red")) {
                return String::from(key);
            } 
            return String::from("");
        })
        .collect();

    test_hm.retain(|x| x != "");
    println!("{:?}", test_hm);

    Ok(
        "{
            \"fc821528d427c906ad1be41e1c5350657ff6bd3f8e60758550cf202161d593f8\":[\"test_tag_1\",\"red\"],
            \"7a95a2b5c3b5ae743c233c138307e3159a44f624fdcda0eec641b37b1859bc80\":[\"test_tag_1\",\"blue\"],
            \"e2c5626baca673c40b41f6a2673bafd3ca02005e8bf21b7d1d78de2ee8690066\":[\"test_tag_2\",\"red\",\"blue\"],
            \"066aa787610eb2eacd2ca207947afd524ae128f02a11c767ec93ac2c5ada3fb1\":[\"red\"],
            \"64bf3282c8b9c8ab81fbf581f97113348fdaa87ae0ba8765bab382a93e87492b\":[\"yellow\"]
        }"
    )

    let j = serde_json::to_string(&images.tags.clone()).unwrap();
    println!("{}", j);
    fs::write(data_file, j).expect("Unable to write file");
```

# in RefImageView implementation update function
## in egui::CentralPanel::default().show()
```rust
let tooltip_ui = |ui: &mut egui::Ui| {
    ui.label(
        egui::RichText::new("Test thingie.."),
    );
    ui.label(format!("More tests.. {}", "yes"));
};

if ui.add(button).on_hover_ui(tooltip_ui).clicked() {
    ui.output().copied_text = chr.to_string();
}
```