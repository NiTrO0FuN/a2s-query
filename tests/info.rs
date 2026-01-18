use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use a2s_query::A2S;
use a2s_query::info::{Info, ServerEnvironment, ServerType, TheShipInfo, TheShipMode};

#[test]
fn test_info_counter_strike_source() {
    let response_data = &[
        0xFF, 0xFF, 0xFF, 0xFF, 0x49, 0x02, 0x67, 0x61, 0x6D, 0x65, 0x32, 0x78, 0x73, 0x2E, 0x63,
        0x6F, 0x6D, 0x20, 0x43, 0x6F, 0x75, 0x6E, 0x74, 0x65, 0x72, 0x2D, 0x53, 0x74, 0x72, 0x69,
        0x6B, 0x65, 0x20, 0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x23, 0x31, 0x00, 0x64, 0x65,
        0x5F, 0x64, 0x75, 0x73, 0x74, 0x00, 0x63, 0x73, 0x74, 0x72, 0x69, 0x6B, 0x65, 0x00, 0x43,
        0x6F, 0x75, 0x6E, 0x74, 0x65, 0x72, 0x2D, 0x53, 0x74, 0x72, 0x69, 0x6B, 0x65, 0x3A, 0x20,
        0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x00, 0xF0, 0x00, 0x05, 0x10, 0x04, 0x64, 0x6C, 0x00,
        0x00, 0x31, 0x2E, 0x30, 0x2E, 0x30, 0x2E, 0x32, 0x32, 0x00,
    ];
    let expected_info = Info {
        protocol: 2,
        name: "game2xs.com Counter-Strike Source #1".to_string(),
        map: "de_dust".to_string(),
        folder: "cstrike".to_string(),
        game: "Counter-Strike: Source".to_string(),
        app_id: 240,
        players: 5,
        max_players: 16,
        bots: 4,
        server_type: ServerType::Dedicated,
        environment: ServerEnvironment::Linux,
        password: false,
        vac: false,
        the_ship: None,
        version: "1.0.0.22".to_string(),
        edf: 0,
        port: None,
        steam_id: None,
        sourcetv_info: None,
        keywords: None,
        game_id: None,
    };
    test_data_info(response_data, &expected_info);
}

#[test]
fn test_info_the_ship() {
    let response_data = &[
        0xFF, 0xFF, 0xFF, 0xFF, 0x49, 0x07, 0x53, 0x68, 0x69, 0x70, 0x20, 0x53, 0x65, 0x72, 0x76,
        0x65, 0x72, 0x00, 0x62, 0x61, 0x74, 0x61, 0x76, 0x69, 0x65, 0x72, 0x00, 0x73, 0x68, 0x69,
        0x70, 0x00, 0x54, 0x68, 0x65, 0x20, 0x53, 0x68, 0x69, 0x70, 0x00, 0x60, 0x09, 0x01, 0x05,
        0x00, 0x6C, 0x77, 0x00, 0x00, 0x01, 0x03, 0x03, 0x31, 0x2E, 0x30, 0x2E, 0x30, 0x2E, 0x34,
        0x00,
    ];
    let expected_info = Info {
        protocol: 7,
        name: "Ship Server".to_string(),
        map: "batavier".to_string(),
        folder: "ship".to_string(),
        game: "The Ship".to_string(),
        app_id: 2400,
        players: 1,
        max_players: 5,
        bots: 0,
        server_type: ServerType::NonDedicated,
        environment: ServerEnvironment::Windows,
        password: false,
        vac: false,
        the_ship: Some(TheShipInfo {
            mode: TheShipMode::Elimination,
            witnesses: 3,
            duration: 3,
        }),
        version: "1.0.0.4".to_string(),
        edf: 0,
        port: None,
        steam_id: None,
        sourcetv_info: None,
        keywords: None,
        game_id: None,
    };
    test_data_info(response_data, &expected_info);
}

#[test]
fn test_info_sin_dm() {
    let response_data = &[
        0xFF, 0xFF, 0xFF, 0xFF, 0x49, 0x2F, 0x53, 0x65, 0x6E, 0x73, 0x65, 0x6D, 0x61, 0x6E, 0x6E,
        0x20, 0x53, 0x69, 0x4E, 0x20, 0x44, 0x4D, 0x00, 0x70, 0x61, 0x72, 0x61, 0x64, 0x6F, 0x78,
        0x00, 0x53, 0x69, 0x4E, 0x20, 0x31, 0x00, 0x53, 0x69, 0x4E, 0x20, 0x31, 0x00, 0x1D, 0x05,
        0x00, 0x10, 0x00, 0x6C, 0x77, 0x00, 0x00, 0x31, 0x2E, 0x30, 0x2E, 0x30, 0x2E, 0x30, 0x00,
    ];
    let expected_info = Info {
        protocol: 47,
        name: "Sensemann SiN DM".to_string(),
        map: "paradox".to_string(),
        folder: "SiN 1".to_string(),
        game: "SiN 1".to_string(),
        app_id: 1309,
        players: 0,
        max_players: 16,
        bots: 0,
        server_type: ServerType::NonDedicated,
        environment: ServerEnvironment::Windows,
        password: false,
        vac: false,
        the_ship: None,
        version: "1.0.0.0".to_string(),
        edf: 0,
        port: None,
        steam_id: None,
        sourcetv_info: None,
        keywords: None,
        game_id: None,
    };
    test_data_info(response_data, &expected_info);
}

fn test_data_info(response_data: &'static [u8], expected_info: &Info) {
    let server_socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind server socket");
    server_socket
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .expect("Failed to set read timeout");
    let server_addr = server_socket
        .local_addr()
        .expect("Failed to get local address");

    let server_handle = thread::spawn(move || {
        let mut buf = [0u8; 25];

        let (_, client_addr) = server_socket
            .recv_from(&mut buf)
            .expect("Failed to receive info request");
        server_socket
            .send_to(&response_data, client_addr)
            .expect("Failed to send info response");
    });

    thread::sleep(Duration::from_millis(10));

    let a2s = A2S::new(server_addr);

    let info = a2s.info().expect("Failed to get info");
    assert_eq!(info, *expected_info);

    server_handle.join().expect("Server thread panicked");
}
