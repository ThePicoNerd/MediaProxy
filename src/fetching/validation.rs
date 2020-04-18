use dns_lookup::lookup_host;
use std::net::IpAddr;
use url::{Host, Url};

fn ip_is_safe(ip: IpAddr) -> bool {
    ip.is_global()
}

pub fn url_is_safe(url: Url) -> bool {
    let scheme = url.scheme();
    let host = url.host();

    match (scheme, host) {
        ("http" | "https", Some(host)) => match host {
            Host::Domain(domain) => match lookup_host(domain) {
                Ok(ips) => {
                    for ip in ips {
                        if !ip_is_safe(ip) {
                            return false;
                        }
                    }

                    true
                }
                Err(_) => false,
            },
            Host::Ipv4(addr) => ip_is_safe(IpAddr::V4(addr)),
            Host::Ipv6(addr) => ip_is_safe(IpAddr::V6(addr)),
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ssrf_urls() {
        let bad_urls = [
            "ftp://127.0.0.1",
            "http://127.0.0.1",
            "https://127.0.0.1",
            "http://localhost",
            "http://10.0.16.33",
            "http://hello.local",
            "http://wow.internal",
        ];
        let good_urls = [
            "http://google.com",
            "https://lynx.agency",
            "https://1.1.1.1",
        ];

        for bad_url in bad_urls.iter() {
            assert_eq!(
                url_is_safe(Url::parse(bad_url).unwrap()),
                false,
                "{}",
                bad_url
            );
        }

        for good_url in good_urls.iter() {
            assert_eq!(
                url_is_safe(Url::parse(good_url).unwrap()),
                true,
                "{}",
                good_url
            );
        }
    }
}
