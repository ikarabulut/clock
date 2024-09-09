# Clock
Basic CLI to get and set system time. As well as view NTP server lag

**There will be a system check for SIP (System Integrity Protection) if it is enabled you will not be able to set time**
## Useage
**Get Time**

You can get the systems clock value based on `Timestamp | rfc2822 | rfc3339`
```
cargo run --
cargo run -- -s rfc2822
cargo run -- -s rfc3339
```

**Set Time**
```
cargo run -- --action set -s rfc3339 {"rfc3339 TIMESTAMP"}
```

**View NTP servers**
Checks lag from
- time.nist.gov
- time.apple.com
- time.euro.apple.com
- time.google.com
- time2.google.com
```
cargo run -- --action check-ntp
```