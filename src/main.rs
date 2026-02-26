use std::fs; // 파일시스템 읽기/쓰기를 위한 표준 라이브러리 모듈 호출
use std::path::Path; // 파일 경로 존재 여부 및 경로 조작 도구 호출

fn main() {
    // K8s(Linux) 환경에서 CPU 제한 값이 기록되는 커널 가상 파일 경로
    // CURI: &str은 뭐지?
    // ANS: 문자열 슬라이스, 시작 주소와 길이를 저장한다.
    // CURI: cgroup은 무슨 뜻이지?
    // ANS: 리눅스 커널의 자원 제어 기술. 프로세스(컨테이너가 아니구나, 머리에 떠오른 것은 cpu.procs) 사용할 수 있는 CPU, memory, 네트워크 대역폭등을 제한하거나 격리한다.
    // ANS: 가상 파일 시스템을 사용해서 구현한 기술.커널이 태생적으로 갖고 있는 기능
    // CURI: .max는 어떤 파일이지? sh, py같은 스크립트? 의미를 모르겠어.
    // ANS: max는 cpu의 사용량? 을 보여주는 가상 파일 시스템. cat을 통해 함수를 호출(open)해서 cpu의 최대 사용시간? 을 확인할 수 있다.
    let cpu_max_path: &str = "/sys/fs/cgroup/cpu.max";
    // CURI: println!이라고 되어있는 ln!은 무슨 뜻이지? 파이썬에서는 그냥 print였는데
    // ANS: ln은 line new,인듯 하다. 줄바꿈 하라는 뜻이다.
    println!("---performance Spy---");

    if Path::new(cpu_max_path).exists() { // CURI: 컨테이너 안에 cpu.max가 가상파일이 있다면, 있다는것은 무슨 뜻이지? 아, local vs container인가보다. local에서는 거꾸로 cgroup바로 아래에 cpu.max가 없는건가? 하지만 linux안에는 cgroup이 있다고 했는데
        match fs::read_to_string(cpu_max_path) {
            Ok(content) => { // CURI: Ok는 무슨 뜻이야?
                let parts: Vec<&str> = content.split_whitespace().collect(); // CURI: Vec은 뭐지?
                if parts.len() >= 2 {
                    let quota = parts[0];
                    let period = parts[1];

                    println!("Quota: {}, Period: {}", quota, period);

                    if quota != "max" {
                        let q: f64 = quota.parse().unwrap_or(0.0);
                        let p: f64 = period.parse().unwrap_or(100000.0);
                        println!("Calculated Limit: {:.2} cores", q / p);
                    } else {
                        println!("Result: Unlimited (No Throttling risk)");
                    }
                }
            }
            Err(e) => println!("Error reading cgroup: {}", e),
        }
    } else {
        println!("Status: Running on Local (Cgroup v2 not found)");
        println!("Note: To test this, we need a Linux container environment");
    }
}


let cpu_max_path :&str = "/sys/fs/cgroup/cpu.max";
println!("---performance Spy---");


// MEMO: 역시 치니까 이해가 되네. 동시에 왜 이게 필요한지, 의문도 들었다. 쿠버네티스에서 왜 이게 필요한지 전체 모양을 알아야(목적) 더 빠르게 만들 수 있겠다. 확인하자.
// CURI: match는 뭐지?
match fs::read_to_string(cpu_max_path)

match fs::read_to_string(cpu_max_path) {
    Ok(content: String) => {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() >= 2 {
            let quota: &str = parts[0];
            let period: &str = parts[1];

            if quota != "max" {
                let q: f64 = quota.parse().unwrap_or(default:0.0);
                let p: f64 = quota.parse().unwrap_or(default: 100000.0);
                println!("Calculated Limit: {:.2} cores", q/p);
            }
        }
    }
    Err(e: Error) => printLn!("Error reading cgroup: {}", e),


