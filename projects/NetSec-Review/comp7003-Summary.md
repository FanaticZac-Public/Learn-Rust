# My Review - COMP 7003 Introduction to Information and Network Security

## Super Summary (or summary of stuff I'd like to deep dive again)

There tools are worth deep diving again
- Using hping3
- Using nmap for scanning machines
- Ncat: Your General-Purpose Network Connector
  - (Swiss Army Knife baby)

Anyway - this was good to get refresher on this course and project. Maybe worth

### Rust related adaptation potentials
- Potentially making my own christmas tree attack
- Manually writing my own packet creator in Rust (or finding using one)
- Using rust to run hping3/nmap/ncat commands as part of a more goal oriented pipeline (For I guess the first 2 parts of the chain in the Cyber Kill Chain)


## Comp7003 - Course Review

This course was very intro course to networking layers, devices, etc.. I'm just going to gloss over most of it, and note anything interesting here.

There was a copy of [Lockheed Martin_s Cyber Kill Chain](https://www.lockheedmartin.com/en-us/capabilities/cyber/cyber-kill-chain.html) - including the phases of a hypothetical attack: Reconnaissance, Weaponization, Delivery, Exploitation, Installation, C2 (Command and Control), Actions on Objectives.

### Topics Covered (I'm just putting this here mostly for my own reference):

- Daemons
- ENV Variables
- PATH
- Bash
- System Logs
- Unix Permissions
- Overview of cybersecurity
- Packet headers
    - Frame
    - Ethernet 2
    - How to read hex dumps
    - Flags
    - IP 
    - TCP 
        - Connection Setup (Three-Way Handshake)
        - Connection Teardown (Four-Way Handshake)
    - UDP (Connectionless)
- Ncat: Your General-Purpose Network Connector
- Basic Password security
- Routing
    - ARP Request and Reply Process
    - ARP Cache and Expiration Time
    - Transport Layer Protocols
    - Subnet Masks and Their Role in Routing Decisions
    - Forwarding Packets to the Destination
    - Packet Delivery Process
    - Network Address Translation (NAT)
    - Advanced Concepts and Further Exploration
        - ARP Spoofing and Security Measures
        - Routing Optimization and Network Topology

### My project for this course

My project included setting up 2 VMs (1 attacker, 1 defender) setup with firewall and IDS systems and then scanning performing network scans for vulnerabilities using hping3, then defending against those attacks and detecting them.

#### Victim

- Using hping3 to understand attack vectors
- Simulating network attacks using hping3
- Implementing firewall rules to prevent such attacks
- Configure an IDS to detect and log attempted attacks.
- Analyze network traffic to verify security measures' effectiveness.

#### Attacker

- Using nmap to scan ports
- Using nmap to get OS info: 'nmap -O IP_ADDR'
- Using nmap to get versions -sV for versions
- Using nmap to scan aggressivesly '-A' (this is more detectable)
- Using nmap with T options to reduce detection

#### hping3 commands ran

- Apart from network congestion - will be focused towards a port - most likely open udp service
  port
- Common service disruption: voip, DNS, media, gaming servers
- Mitigation: blocking udp traffic on unnecessary open ports, monitoring
- Sending 10000 UDP packets with a payload size of 120 bytes to port 22 to Linux1

sudo hping3 -c 10000 -d 120 -S -w 64 -p 22 --udp 192.168.43.187

##### UDP flood

- Apart from network congestion - will be focused towards a port - - most likely open udp service port
- Common service disruption: voip, DNS, media, gaming servers
  Mitigation: blocking udp traffic on unnecessary open ports, - monitoring
- Sending 10000 UDP packets with a payload size of 120 bytes to port - 22 to Linux1

sudo hping3 -c 10000 -d 120 -S -w 64 -p 22 --udp 192.168.43.187

##### ICMP flood

- Symmetrical Attack - Replies in equal ratio of sending
- Can do source spoofing
- Large rate of requests
- DOS attack focused on congesting network infrastructure and - environments rather than ports or services directly
- Monitoring network environment

sudo hping3 -c 10000 -d 120 -S -w 64 --icmp 192.168.43.187

##### SYN Flood

- This attack sends a bunch of SYN packet and while the server is responding - believing its setting up a ton of connections - it’s leaving a bunch of connections half-open, using up resources. (rand-source is
  spoofing)
- Rapid consumption of resources (memory, CPU, and network bandwidth)
- Manipulates the TCP handshake process
- Common Victims (web servers, email servers, database servers, and application servers). Essentially, any server that handles incoming TCP connections

sudo hping3 -c 50000 -d 1000 --rand-source -q -i u200 --syn 192.168.43.187

##### Unsolicited SYN/ACK

- Resource consumption
  sudo hping3 -c 50000 -d 1000 --rand-source -q --ack 192.168.43.187

##### Christmas Tree Attack

- I ran this attack via hping3 - but really it should be scripted to run all possible combinations of packet header flags...
- Checks all combinations for any that may give access to a port
- to then be used for further ddos or other attack entry points
- hence like the flashing random flashing lights on a christmas tree

sudo hping3 -c 50000 -d 1000 --rand-source -q -F -S -R -P -U -A -X 192.168.43.187

#### Defender

My working combinations

UDP flood + ICMP flood

- sudo nft add rule ip filter input tcp dport {22, 80, 443} accept
  Drop everything else
- sudo nft add rule ip filter input drop
  SYN Flood,
  sudo nft add rule ip filter input tcp flags syn limit rate 10/- second accept
  Unsolicited SYN/ACK
- sudo nft add rule ip filter input tcp flags { syn, ack } drop
  Christmas Tree Attack
  sudo nft add rule ip filter input tcp flags { fin, syn, rst, psh, - ack, urg } drop

```console
sudo nft add table ip filter
sudo nft flush chain filter output
sudo nft add chain ip filter output { type filter hook output priority 0 \; }
sudo nft add chain ip filter output { type filter hook output priority 0 \; policy drop \; }
sudo nft list ruleset
table ip filter {
    chain output {
        type filter hook output priority filter; policy drop;
    }
}
```

```console
table ip filter {
    chain output {
        type filter hook output priority filter; policy drop;
        tcp flags syn limit rate 3/minute drop
        tcp dport { 22, 80, 443 } accept
        tcp flags { syn, ack } ct state new drop
        tcp flags { fin, syn, rst, psh, ack, urg } drop
        drop
    }
}
```

#### IDS setup

sudo apt-get install suricata
// as it’s built into apt and easy to use
Here is final confirmed working suricata.rules:# Detects UDP Flood
alert udp any any -> any any (msg:"UDP Flood Detected"; threshold: type threshold, track by_src, count
100, seconds 30; sid:100001; rev:1;)

##### Detects ICMP Flood

alert icmp any any -> any any (msg:"ICMP Flood Detected"; threshold: type limit, track by_dst, count
100, seconds 60; sid:100002; rev:1;)

##### Detects SYN Flood

alert tcp any any -> any 22 (msg:"SYN Flood Detected"; flags:S; threshold: type threshold, track by_src,
count 100, seconds 30; sid:100003; rev:1; flow:to_server;)

##### Detects Unsolicited SYN/ACK

alert tcp any any -> any 22 (msg:"Unsolicited SYN/ACK Detected"; flags:SA; threshold: type threshold,
track by_src, count 1, seconds 60; sid:100004; rev:1;)

##### Detects Christmas Tree Attack

alert tcp any any -> any 22 (msg:"Christmas Tree Attack Detected"; flags:SFPUAR; threshold: type
threshold, track by_src, count 1, seconds 60; sid:100005; rev:1;)
