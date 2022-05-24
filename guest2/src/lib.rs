wit_bindgen_rust::export!("../wits/say.wit");

struct Say;

impl say::Say for Say {
    fn hello(name: String) -> String {
        let rc = if name != "Michael" {
            format!("goodbye {}", name)
        } else {
            format!("hello {}", name)
        };

        rc
    }

    fn overhead(name: String) -> (String, u64) {
        let s = std::time::Instant::now();
        let rc = format!("goodbye {}", name);
        let ms = s.elapsed().as_nanos();

        (rc, ms as u64)
    }
}

