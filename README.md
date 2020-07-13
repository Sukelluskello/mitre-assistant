# mitre-assistant

WIP - Stay tuned

## Current Progress
If you decide to compile and use it in current status, you can only run this in Linux or a MacOS, because the current codebase writes the resultant output to the **home** folder under a dot directory **.mitre-assistant**.
<br/>

## CLI Usage

```bash
# Main Menu
# Uses a Subcommand Modular Approach
#

$> mitre-assistant -h

# Output

mitre-assistant v.0.0.1
carlos diaz | @dfirence

Mitre Attack Assistant

USAGE:
    mitre-assistant [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    baseline    Parse a Matrix into comprehensive insights
    download    A more useful utility for the ATT&CK Matrix
    help        Prints this message or the help of the given subcommand(s)

```
<br/>
<br/>

### Download Mode

```bash
# Download Subcommand
#
$> mitre-assistant download -h 

# Output
mitre-assistant-download v.0.0.1
carlos diaz | @dfirence

A more useful utility for the ATT&CK Matrix

USAGE:
    mitre-assistant download [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --matrix <matrix_name>    Load a Matrix From ATT&CK: (Enterprise|Mobile|Pre-Attatck)

```
<br/>

```bash
# Download the enterprise matrix
$> mitre-assistant download -m enterprise

# Or Download the mobile matrix
$> mitre-assistant download -m mobile

# Or Download the pre-attack matrix
$> mitre-assistant download -m pre-attack
```
<br/>
<br/>

### Baseline Mode
Baseline mode **requires** that you have **downloaded** the matrix you want to baseline.  A baseline means, acquiring specific information elements from the matrix, transforming these into smaller datasets so you can actually ask specific questions - independently - for your needs - e.g., *how many windows techniques in the collection tactic use the process monitoring datasource?*


```bash
# Baseline Subcommand
#
$> mitre-assistant baseline -h

# Output
mitre-assistant-baseline v.0.0.1
carlos diaz | @dfirence

Parse a Matrix into comprehensive insights

USAGE:
    mitre-assistant baseline [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --matrix <matrix_name>    Load a Matrix From ATT&CK: (Enterprise|Mobile|Pre-Attatck)

```
<br/>

## Why Even Bother
I work in the Security Industry for a Service Provider, and the amount of work we have to do to work with the Mitre Matrix is significant.  As a consequence, I have to constantly check the status of the matrix, and ask questions of its data - continously.

I have seen many cool utilities from other folks whose intent is the same - simplify this matrix for daily usage.  I have used some of their tools, and haven't seen what I am looking for.  I also don't want to always install an interpreted language just to get some basic information from the JSON files of the matrix, and lastly, I also don't find the website useful for people that have to achieve work with the framework - clicking links is not fun.

So in the end, I rather create my own once and for all.

## Data Interchange Formats
It is my intent to provide CSV and JSON... **again**, CSV **and** JSON (both).


## Current Data Sample
The baseline module produces this for the enterprise matrix - more to go.

```bash
# Snapshot 2020-07-12
# Mitre once again changed their CTI
#
# This command must have downloaded the matrix already
$> mitre-assistant baseline -m enterprise
```
<br/>


```json
{
  "techniques": [
    [
      "T1538",
      "Cloud Service Dashboard",
      "discovery"
    ],
    [
      "T1137",
      "Office Application Startup",
      "persistence"
    ],
    [
      "T1176",
      "Browser Extensions",
      "persistence"
    ],
    [
      "T1200",
      "Hardware Additions",
      "initial-access"
    ],
    [
      "T1040",
      "Network Sniffing",
      "discovery"
    ],
    [
      "T1543",
      "Create or Modify System Process",
      "persistence"
    ],
    [
      "T1546",
      "Event Triggered Execution",
      "persistence"
    ],
    [
      "T1557",
      "Man-in-the-Middle",
      "collection"
    ],
    [
      "T1030",
      "Data Transfer Size Limits",
      "exfiltration"
    ],
    [
      "T1104",
      "Multi-Stage Channels",
      "command-and-control"
    ],
    [
      "T1202",
      "Indirect Command Execution",
      "defense-evasion"
    ],
    [
      "T1219",
      "Remote Access Software",
      "command-and-control"
    ],
    [
      "T1052",
      "Exfiltration Over Physical Medium",
      "exfiltration"
    ],
    [
      "T1539",
      "Steal Web Session Cookie",
      "credential-access"
    ],
    [
      "T1091",
      "Replication Through Removable Media",
      "lateral-movement"
    ],
    [
      "T1218",
      "Signed Binary Proxy Execution",
      "defense-evasion"
    ],
    [
      "T1153",
      "Source",
      "execution"
    ],
    [
      "T1526",
      "Cloud Service Discovery",
      "discovery"
    ],
    [
      "T1149",
      "LC_MAIN Hijacking",
      "defense-evasion"
    ],
    [
      "T1072",
      "Software Deployment Tools",
      "execution"
    ],
    [
      "T1221",
      "Template Injection",
      "defense-evasion"
    ],
    [
      "T1207",
      "Rogue Domain Controller",
      "defense-evasion"
    ],
    [
      "T1078",
      "Valid Accounts",
      "initial-access"
    ],
    [
      "T1497",
      "Virtualization/Sandbox Evasion",
      "discovery"
    ],
    [
      "T1047",
      "Windows Management Instrumentation",
      "execution"
    ],
    [
      "T1051",
      "Shared Webroot",
      "lateral-movement"
    ],
    [
      "T1550",
      "Use Alternate Authentication Material",
      "lateral-movement"
    ],
    [
      "T1554",
      "Compromise Client Software Binary",
      "persistence"
    ],
    [
      "T1005",
      "Data from Local System",
      "collection"
    ],
    [
      "T1092",
      "Communication Through Removable Media",
      "command-and-control"
    ],
    [
      "T1014",
      "Rootkit",
      "defense-evasion"
    ],
    [
      "T1053",
      "Scheduled Task/Job",
      "execution"
    ],
    [
      "T1053",
      "Scheduled Task/Job",
      "privilege-escalation"
    ],
    [
      "T1569",
      "System Services",
      "execution"
    ],
    [
      "T1190",
      "Exploit Public-Facing Application",
      "initial-access"
    ],
    [
      "T1062",
      "Hypervisor",
      "persistence"
    ],
    [
      "T1120",
      "Peripheral Device Discovery",
      "discovery"
    ],
    [
      "T1525",
      "Implant Container Image",
      "persistence"
    ],
    [
      "T1025",
      "Data from Removable Media",
      "collection"
    ],
    [
      "T1113",
      "Screen Capture",
      "collection"
    ],
    [
      "T1573",
      "Encrypted Channel",
      "command-and-control"
    ],
    [
      "T1070",
      "Indicator Removal on Host",
      "defense-evasion"
    ],
    [
      "T1489",
      "Service Stop",
      "impact"
    ],
    [
      "T1546",
      "Event Triggered Execution",
      "privilege-escalation"
    ],
    [
      "T1037",
      "Boot or Logon Initialization Scripts",
      "privilege-escalation"
    ],
    [
      "T1216",
      "Signed Script Proxy Execution",
      "defense-evasion"
    ],
    [
      "T1565",
      "Data Manipulation",
      "impact"
    ],
    [
      "T1197",
      "BITS Jobs",
      "persistence"
    ],
    [
      "T1036",
      "Masquerading",
      "defense-evasion"
    ],
    [
      "T1087",
      "Account Discovery",
      "discovery"
    ],
    [
      "T1542",
      "Pre-OS Boot",
      "defense-evasion"
    ],
    [
      "T1212",
      "Exploitation for Credential Access",
      "credential-access"
    ],
    [
      "T1195",
      "Supply Chain Compromise",
      "initial-access"
    ],
    [
      "T1491",
      "Defacement",
      "impact"
    ],
    [
      "T1187",
      "Forced Authentication",
      "credential-access"
    ],
    [
      "T1056",
      "Input Capture",
      "collection"
    ],
    [
      "T1090",
      "Proxy",
      "command-and-control"
    ],
    [
      "T1029",
      "Scheduled Transfer",
      "exfiltration"
    ],
    [
      "T1537",
      "Transfer Data to Cloud Account",
      "exfiltration"
    ],
    [
      "T1564",
      "Hide Artifacts",
      "defense-evasion"
    ],
    [
      "T1574",
      "Hijack Execution Flow",
      "defense-evasion"
    ],
    [
      "T1135",
      "Network Share Discovery",
      "discovery"
    ],
    [
      "T1136",
      "Create Account",
      "persistence"
    ],
    [
      "T1110",
      "Brute Force",
      "credential-access"
    ],
    [
      "T1574",
      "Hijack Execution Flow",
      "privilege-escalation"
    ],
    [
      "T1557",
      "Man-in-the-Middle",
      "credential-access"
    ],
    [
      "T1011",
      "Exfiltration Over Other Network Medium",
      "exfiltration"
    ],
    [
      "T1556",
      "Modify Authentication Process",
      "defense-evasion"
    ],
    [
      "T1069",
      "Permission Groups Discovery",
      "discovery"
    ],
    [
      "T1106",
      "Native API",
      "execution"
    ],
    [
      "T1499",
      "Endpoint Denial of Service",
      "impact"
    ],
    [
      "T1108",
      "Redundant Access",
      "persistence"
    ],
    [
      "T1486",
      "Data Encrypted for Impact",
      "impact"
    ],
    [
      "T1555",
      "Credentials from Password Stores",
      "credential-access"
    ],
    [
      "T1571",
      "Non-Standard Port",
      "command-and-control"
    ],
    [
      "T1518",
      "Software Discovery",
      "discovery"
    ],
    [
      "T1547",
      "Boot or Logon Autostart Execution",
      "privilege-escalation"
    ],
    [
      "T1531",
      "Account Access Removal",
      "impact"
    ],
    [
      "T1115",
      "Clipboard Data",
      "collection"
    ],
    [
      "T1132",
      "Data Encoding",
      "command-and-control"
    ],
    [
      "T1039",
      "Data from Network Shared Drive",
      "collection"
    ],
    [
      "T1140",
      "Deobfuscate/Decode Files or Information",
      "defense-evasion"
    ],
    [
      "T1189",
      "Drive-by Compromise",
      "initial-access"
    ],
    [
      "T1495",
      "Firmware Corruption",
      "impact"
    ],
    [
      "T1027",
      "Obfuscated Files or Information",
      "defense-evasion"
    ],
    [
      "T1563",
      "Remote Service Session Hijacking",
      "lateral-movement"
    ],
    [
      "T1021",
      "Remote Services",
      "lateral-movement"
    ],
    [
      "T1129",
      "Shared Modules",
      "execution"
    ],
    [
      "T1542",
      "Pre-OS Boot",
      "persistence"
    ],
    [
      "T1543",
      "Create or Modify System Process",
      "privilege-escalation"
    ],
    [
      "T1048",
      "Exfiltration Over Alternative Protocol",
      "exfiltration"
    ],
    [
      "T1199",
      "Trusted Relationship",
      "initial-access"
    ],
    [
      "T1001",
      "Data Obfuscation",
      "command-and-control"
    ],
    [
      "T1012",
      "Query Registry",
      "discovery"
    ],
    [
      "T1547",
      "Boot or Logon Autostart Execution",
      "persistence"
    ],
    [
      "T1217",
      "Browser Bookmark Discovery",
      "discovery"
    ],
    [
      "T1083",
      "File and Directory Discovery",
      "discovery"
    ],
    [
      "T1211",
      "Exploitation for Defense Evasion",
      "defense-evasion"
    ],
    [
      "T1535",
      "Unused/Unsupported Cloud Regions",
      "defense-evasion"
    ],
    [
      "T1497",
      "Virtualization/Sandbox Evasion",
      "defense-evasion"
    ],
    [
      "T1105",
      "Ingress Tool Transfer",
      "command-and-control"
    ],
    [
      "T1078",
      "Valid Accounts",
      "persistence"
    ],
    [
      "T1570",
      "Lateral Tool Transfer",
      "lateral-movement"
    ],
    [
      "T1020",
      "Automated Exfiltration",
      "exfiltration"
    ],
    [
      "T1530",
      "Data from Cloud Storage Object",
      "collection"
    ],
    [
      "T1568",
      "Dynamic Resolution",
      "command-and-control"
    ],
    [
      "T1068",
      "Exploitation for Privilege Escalation",
      "privilege-escalation"
    ],
    [
      "T1008",
      "Fallback Channels",
      "command-and-control"
    ],
    [
      "T1578",
      "Modify Cloud Compute Infrastructure",
      "defense-evasion"
    ],
    [
      "T1133",
      "External Remote Services",
      "persistence"
    ],
    [
      "T1040",
      "Network Sniffing",
      "credential-access"
    ],
    [
      "T1222",
      "File and Directory Permissions Modification",
      "defense-evasion"
    ],
    [
      "T1213",
      "Data from Information Repositories",
      "collection"
    ],
    [
      "T1108",
      "Redundant Access",
      "defense-evasion"
    ],
    [
      "T1018",
      "Remote System Discovery",
      "discovery"
    ],
    [
      "T1560",
      "Archive Collected Data",
      "collection"
    ],
    [
      "T1496",
      "Resource Hijacking",
      "impact"
    ],
    [
      "T1574",
      "Hijack Execution Flow",
      "persistence"
    ],
    [
      "T1482",
      "Domain Trust Discovery",
      "discovery"
    ],
    [
      "T1010",
      "Application Window Discovery",
      "discovery"
    ],
    [
      "T1041",
      "Exfiltration Over C2 Channel",
      "exfiltration"
    ],
    [
      "T1204",
      "User Execution",
      "execution"
    ],
    [
      "T1203",
      "Exploitation for Client Execution",
      "execution"
    ],
    [
      "T1134",
      "Access Token Manipulation",
      "defense-evasion"
    ],
    [
      "T1123",
      "Audio Capture",
      "collection"
    ],
    [
      "T1080",
      "Taint Shared Content",
      "lateral-movement"
    ],
    [
      "T1007",
      "System Service Discovery",
      "discovery"
    ],
    [
      "T1528",
      "Steal Application Access Token",
      "credential-access"
    ],
    [
      "T1558",
      "Steal or Forge Kerberos Tickets",
      "credential-access"
    ],
    [
      "T1049",
      "System Network Connections Discovery",
      "discovery"
    ],
    [
      "T1078",
      "Valid Accounts",
      "defense-evasion"
    ],
    [
      "T1559",
      "Inter-Process Communication",
      "execution"
    ],
    [
      "T1057",
      "Process Discovery",
      "discovery"
    ],
    [
      "T1006",
      "Direct Volume Access",
      "defense-evasion"
    ],
    [
      "T1556",
      "Modify Authentication Process",
      "credential-access"
    ],
    [
      "T1548",
      "Abuse Elevation Control Mechanism",
      "defense-evasion"
    ],
    [
      "T1119",
      "Automated Collection",
      "collection"
    ],
    [
      "T1114",
      "Email Collection",
      "collection"
    ],
    [
      "T1534",
      "Internal Spearphishing",
      "lateral-movement"
    ],
    [
      "T1185",
      "Man in the Browser",
      "collection"
    ],
    [
      "T1220",
      "XSL Script Processing",
      "defense-evasion"
    ],
    [
      "T1197",
      "BITS Jobs",
      "defense-evasion"
    ],
    [
      "T1046",
      "Network Service Scanning",
      "discovery"
    ],
    [
      "T1091",
      "Replication Through Removable Media",
      "initial-access"
    ],
    [
      "T1061",
      "Graphical User Interface",
      "execution"
    ],
    [
      "T1026",
      "Multiband Communication",
      "command-and-control"
    ],
    [
      "T1572",
      "Protocol Tunneling",
      "command-and-control"
    ],
    [
      "T1490",
      "Inhibit System Recovery",
      "impact"
    ],
    [
      "T1074",
      "Data Staged",
      "collection"
    ],
    [
      "T1484",
      "Group Policy Modification",
      "defense-evasion"
    ],
    [
      "T1102",
      "Web Service",
      "command-and-control"
    ],
    [
      "T1071",
      "Application Layer Protocol",
      "command-and-control"
    ],
    [
      "T1053",
      "Scheduled Task/Job",
      "persistence"
    ],
    [
      "T1124",
      "System Time Discovery",
      "discovery"
    ],
    [
      "T1003",
      "OS Credential Dumping",
      "credential-access"
    ],
    [
      "T1567",
      "Exfiltration Over Web Service",
      "exfiltration"
    ],
    [
      "T1127",
      "Trusted Developer Utilities Proxy Execution",
      "defense-evasion"
    ],
    [
      "T1529",
      "System Shutdown/Reboot",
      "impact"
    ],
    [
      "T1561",
      "Disk Wipe",
      "impact"
    ],
    [
      "T1037",
      "Boot or Logon Initialization Scripts",
      "persistence"
    ],
    [
      "T1056",
      "Input Capture",
      "credential-access"
    ],
    [
      "T1112",
      "Modify Registry",
      "defense-evasion"
    ],
    [
      "T1480",
      "Execution Guardrails",
      "defense-evasion"
    ],
    [
      "T1210",
      "Exploitation of Remote Services",
      "lateral-movement"
    ],
    [
      "T1498",
      "Network Denial of Service",
      "impact"
    ],
    [
      "T1095",
      "Non-Application Layer Protocol",
      "command-and-control"
    ],
    [
      "T1055",
      "Process Injection",
      "defense-evasion"
    ],
    [
      "T1484",
      "Group Policy Modification",
      "privilege-escalation"
    ],
    [
      "T1562",
      "Impair Defenses",
      "defense-evasion"
    ],
    [
      "T1485",
      "Data Destruction",
      "impact"
    ],
    [
      "T1548",
      "Abuse Elevation Control Mechanism",
      "privilege-escalation"
    ],
    [
      "T1566",
      "Phishing",
      "initial-access"
    ],
    [
      "T1055",
      "Process Injection",
      "privilege-escalation"
    ],
    [
      "T1016",
      "System Network Configuration Discovery",
      "discovery"
    ],
    [
      "T1082",
      "System Information Discovery",
      "discovery"
    ],
    [
      "T1111",
      "Two-Factor Authentication Interception",
      "credential-access"
    ],
    [
      "T1133",
      "External Remote Services",
      "initial-access"
    ],
    [
      "T1134",
      "Access Token Manipulation",
      "privilege-escalation"
    ],
    [
      "T1201",
      "Password Policy Discovery",
      "discovery"
    ],
    [
      "T1552",
      "Unsecured Credentials",
      "credential-access"
    ],
    [
      "T1098",
      "Account Manipulation",
      "persistence"
    ],
    [
      "T1550",
      "Use Alternate Authentication Material",
      "defense-evasion"
    ],
    [
      "T1078",
      "Valid Accounts",
      "privilege-escalation"
    ],
    [
      "T1033",
      "System Owner/User Discovery",
      "discovery"
    ],
    [
      "T1072",
      "Software Deployment Tools",
      "lateral-movement"
    ],
    [
      "T1125",
      "Video Capture",
      "collection"
    ],
    [
      "T1505",
      "Server Software Component",
      "persistence"
    ]
  ],
  "tactics": [
    "defense-evasion",
    "initial-access",
    "impact",
    "privilege-escalation",
    "command-and-control",
    "collection",
    "lateral-movement",
    "credential-access",
    "persistence",
    "discovery",
    "execution",
    "exfiltration"
  ],
  "subtechniques": [
    [
      "T1550.001",
      "Application Access Token",
      "lateral-movement"
    ],
    [
      "T1574.005",
      "Executable Installer File Permissions Weakness",
      "defense-evasion"
    ],
    [
      "T1037.003",
      "Network Logon Script",
      "privilege-escalation"
    ],
    [
      "T1564.002",
      "Hidden Users",
      "defense-evasion"
    ],
    [
      "T1518.001",
      "Security Software Discovery",
      "discovery"
    ],
    [
      "T1562.003",
      "HISTCONTROL",
      "defense-evasion"
    ],
    [
      "T1497.002",
      "User Activity Based Checks",
      "defense-evasion"
    ],
    [
      "T1546.010",
      "AppInit DLLs",
      "persistence"
    ],
    [
      "T1059.005",
      "Visual Basic",
      "execution"
    ],
    [
      "T1003.006",
      "DCSync",
      "credential-access"
    ],
    [
      "T1574.001",
      "DLL Search Order Hijacking",
      "persistence"
    ],
    [
      "T1070.001",
      "Clear Windows Event Logs",
      "defense-evasion"
    ],
    [
      "T1499.003",
      "Application Exhaustion Flood",
      "impact"
    ],
    [
      "T1078.002",
      "Domain Accounts",
      "persistence"
    ],
    [
      "T1546.009",
      "AppCert DLLs",
      "persistence"
    ],
    [
      "T1561.002",
      "Disk Structure Wipe",
      "impact"
    ],
    [
      "T1056.002",
      "GUI Input Capture",
      "collection"
    ],
    [
      "T1027.004",
      "Compile After Delivery",
      "defense-evasion"
    ],
    [
      "T1499.004",
      "Application or System Exploitation",
      "impact"
    ],
    [
      "T1213.001",
      "Confluence",
      "collection"
    ],
    [
      "T1053.003",
      "Cron",
      "persistence"
    ],
    [
      "T1550.002",
      "Pass the Hash",
      "defense-evasion"
    ],
    [
      "T1055.009",
      "Proc Memory",
      "defense-evasion"
    ],
    [
      "T1556.001",
      "Domain Controller Authentication",
      "credential-access"
    ],
    [
      "T1546.014",
      "Emond",
      "privilege-escalation"
    ],
    [
      "T1078.001",
      "Default Accounts",
      "initial-access"
    ],
    [
      "T1204.002",
      "Malicious File",
      "execution"
    ],
    [
      "T1059.006",
      "Python",
      "execution"
    ],
    [
      "T1547.001",
      "Registry Run Keys / Startup Folder",
      "privilege-escalation"
    ],
    [
      "T1574.008",
      "Path Interception by Search Order Hijacking",
      "persistence"
    ],
    [
      "T1218.011",
      "Rundll32",
      "defense-evasion"
    ],
    [
      "T1556.002",
      "Password Filter DLL",
      "credential-access"
    ],
    [
      "T1548.001",
      "Setuid and Setgid",
      "defense-evasion"
    ],
    [
      "T1574.004",
      "Dylib Hijacking",
      "persistence"
    ],
    [
      "T1037.005",
      "Startup Items",
      "privilege-escalation"
    ],
    [
      "T1134.002",
      "Create Process with Token",
      "privilege-escalation"
    ],
    [
      "T1055.002",
      "Portable Executable Injection",
      "privilege-escalation"
    ],
    [
      "T1078.004",
      "Cloud Accounts",
      "initial-access"
    ],
    [
      "T1021.003",
      "Distributed Component Object Model",
      "lateral-movement"
    ],
    [
      "T1550.002",
      "Pass the Hash",
      "lateral-movement"
    ],
    [
      "T1497.003",
      "Time Based Evasion",
      "discovery"
    ],
    [
      "T1547.003",
      "Time Providers",
      "persistence"
    ],
    [
      "T1555.003",
      "Credentials from Web Browsers",
      "credential-access"
    ],
    [
      "T1136.003",
      "Cloud Account",
      "persistence"
    ],
    [
      "T1071.004",
      "DNS",
      "command-and-control"
    ],
    [
      "T1499.002",
      "Service Exhaustion Flood",
      "impact"
    ],
    [
      "T1552.002",
      "Credentials in Registry",
      "credential-access"
    ],
    [
      "T1070.005",
      "Network Share Connection Removal",
      "defense-evasion"
    ],
    [
      "T1098.004",
      "SSH Authorized Keys",
      "persistence"
    ],
    [
      "T1547.007",
      "Re-opened Applications",
      "persistence"
    ],
    [
      "T1548.002",
      "Bypass User Access Control",
      "privilege-escalation"
    ],
    [
      "T1071.002",
      "File Transfer Protocols",
      "command-and-control"
    ],
    [
      "T1222.001",
      "Windows File and Directory Permissions Modification",
      "defense-evasion"
    ],
    [
      "T1548.004",
      "Elevated Execution with Prompt",
      "privilege-escalation"
    ],
    [
      "T1546.003",
      "Windows Management Instrumentation Event Subscription",
      "persistence"
    ],
    [
      "T1052.001",
      "Exfiltration over USB",
      "exfiltration"
    ],
    [
      "T1069.003",
      "Cloud Groups",
      "discovery"
    ],
    [
      "T1003.003",
      "NTDS",
      "credential-access"
    ],
    [
      "T1205.001",
      "Port Knocking",
      "defense-evasion"
    ],
    [
      "T1027.003",
      "Steganography",
      "defense-evasion"
    ],
    [
      "T1056.003",
      "Web Portal Capture",
      "credential-access"
    ],
    [
      "T1071.001",
      "Web Protocols",
      "command-and-control"
    ],
    [
      "T1547.002",
      "Authentication Package",
      "privilege-escalation"
    ],
    [
      "T1137.006",
      "Add-ins",
      "persistence"
    ],
    [
      "T1048.003",
      "Exfiltration Over Unencrypted/Obfuscated Non-C2 Protocol",
      "exfiltration"
    ],
    [
      "T1037.001",
      "Logon Script (Windows)",
      "privilege-escalation"
    ],
    [
      "T1556.003",
      "Pluggable Authentication Modules",
      "defense-evasion"
    ],
    [
      "T1055.012",
      "Process Hollowing",
      "privilege-escalation"
    ],
    [
      "T1546.010",
      "AppInit DLLs",
      "privilege-escalation"
    ],
    [
      "T1578.002",
      "Create Cloud Instance",
      "defense-evasion"
    ],
    [
      "T1098.001",
      "Additional Azure Service Principal Credentials",
      "persistence"
    ],
    [
      "T1037.003",
      "Network Logon Script",
      "persistence"
    ],
    [
      "T1053.005",
      "Scheduled Task",
      "persistence"
    ],
    [
      "T1505.002",
      "Transport Agent",
      "persistence"
    ],
    [
      "T1542.003",
      "Bootkit",
      "defense-evasion"
    ],
    [
      "T1578.001",
      "Create Snapshot",
      "defense-evasion"
    ],
    [
      "T1547.008",
      "LSASS Driver",
      "privilege-escalation"
    ],
    [
      "T1547.004",
      "Winlogon Helper DLL",
      "persistence"
    ],
    [
      "T1548.003",
      "Sudo and Sudo Caching",
      "defense-evasion"
    ],
    [
      "T1078.001",
      "Default Accounts",
      "persistence"
    ],
    [
      "T1003.005",
      "Cached Domain Credentials",
      "credential-access"
    ],
    [
      "T1055.001",
      "Dynamic-link Library Injection",
      "defense-evasion"
    ],
    [
      "T1558.001",
      "Golden Ticket",
      "credential-access"
    ],
    [
      "T1053.004",
      "Launchd",
      "privilege-escalation"
    ],
    [
      "T1205.001",
      "Port Knocking",
      "persistence"
    ],
    [
      "T1021.001",
      "Remote Desktop Protocol",
      "lateral-movement"
    ],
    [
      "T1574.012",
      "COR_PROFILER",
      "privilege-escalation"
    ],
    [
      "T1055.005",
      "Thread Local Storage",
      "defense-evasion"
    ],
    [
      "T1053.002",
      "At (Windows)",
      "privilege-escalation"
    ],
    [
      "T1037.002",
      "Logon Script (Mac)",
      "privilege-escalation"
    ],
    [
      "T1132.001",
      "Standard Encoding",
      "command-and-control"
    ],
    [
      "T1543.004",
      "Launch Daemon",
      "privilege-escalation"
    ],
    [
      "T1021.005",
      "VNC",
      "lateral-movement"
    ],
    [
      "T1059.003",
      "Windows Command Shell",
      "execution"
    ],
    [
      "T1087.002",
      "Domain Account",
      "discovery"
    ],
    [
      "T1573.001",
      "Symmetric Cryptography",
      "command-and-control"
    ],
    [
      "T1055.003",
      "Thread Execution Hijacking",
      "defense-evasion"
    ],
    [
      "T1070.003",
      "Clear Command History",
      "defense-evasion"
    ],
    [
      "T1098.003",
      "Add Office 365 Global Administrator Role",
      "persistence"
    ],
    [
      "T1547.005",
      "Security Support Provider",
      "privilege-escalation"
    ],
    [
      "T1055.005",
      "Thread Local Storage",
      "privilege-escalation"
    ],
    [
      "T1564.005",
      "Hidden File System",
      "defense-evasion"
    ],
    [
      "T1053.001",
      "At (Linux)",
      "privilege-escalation"
    ],
    [
      "T1222.002",
      "Linux and Mac File and Directory Permissions Modification",
      "defense-evasion"
    ],
    [
      "T1055.012",
      "Process Hollowing",
      "defense-evasion"
    ],
    [
      "T1546.001",
      "Change Default File Association",
      "persistence"
    ],
    [
      "T1546.015",
      "Component Object Model Hijacking",
      "persistence"
    ],
    [
      "T1563.001",
      "SSH Hijacking",
      "lateral-movement"
    ],
    [
      "T1574.012",
      "COR_PROFILER",
      "defense-evasion"
    ],
    [
      "T1195.002",
      "Compromise Software Supply Chain",
      "initial-access"
    ],
    [
      "T1558.003",
      "Kerberoasting",
      "credential-access"
    ],
    [
      "T1134.002",
      "Create Process with Token",
      "defense-evasion"
    ],
    [
      "T1056.002",
      "GUI Input Capture",
      "credential-access"
    ],
    [
      "T1497.001",
      "System Checks",
      "defense-evasion"
    ],
    [
      "T1055.001",
      "Dynamic-link Library Injection",
      "privilege-escalation"
    ],
    [
      "T1048.002",
      "Exfiltration Over Asymmetric Encrypted Non-C2 Protocol",
      "exfiltration"
    ],
    [
      "T1564.004",
      "NTFS File Attributes",
      "defense-evasion"
    ],
    [
      "T1003.008",
      "/etc/passwd and /etc/shadow",
      "credential-access"
    ],
    [
      "T1574.011",
      "Services Registry Permissions Weakness",
      "persistence"
    ],
    [
      "T1546.011",
      "Application Shimming",
      "privilege-escalation"
    ],
    [
      "T1552.005",
      "Cloud Instance Metadata API",
      "credential-access"
    ],
    [
      "T1497.003",
      "Time Based Evasion",
      "defense-evasion"
    ],
    [
      "T1546.007",
      "Netsh Helper DLL",
      "privilege-escalation"
    ],
    [
      "T1578.003",
      "Delete Cloud Instance",
      "defense-evasion"
    ],
    [
      "T1078.001",
      "Default Accounts",
      "privilege-escalation"
    ],
    [
      "T1499.001",
      "OS Exhaustion Flood",
      "impact"
    ],
    [
      "T1056.001",
      "Keylogging",
      "credential-access"
    ],
    [
      "T1547.010",
      "Port Monitors",
      "persistence"
    ],
    [
      "T1553.001",
      "Gatekeeper Bypass",
      "defense-evasion"
    ],
    [
      "T1547.009",
      "Shortcut Modification",
      "persistence"
    ],
    [
      "T1564.003",
      "Hidden Window",
      "defense-evasion"
    ],
    [
      "T1564.006",
      "Run Virtual Instance",
      "defense-evasion"
    ],
    [
      "T1543.003",
      "Windows Service",
      "persistence"
    ],
    [
      "T1562.007",
      "Disable or Modify Cloud Firewall",
      "defense-evasion"
    ],
    [
      "T1055.011",
      "Extra Window Memory Injection",
      "defense-evasion"
    ],
    [
      "T1055.008",
      "Ptrace System Calls",
      "defense-evasion"
    ],
    [
      "T1547.009",
      "Shortcut Modification",
      "privilege-escalation"
    ],
    [
      "T1134.004",
      "Parent PID Spoofing",
      "defense-evasion"
    ],
    [
      "T1195.001",
      "Compromise Software Dependencies and Development Tools",
      "initial-access"
    ],
    [
      "T1574.006",
      "LD_PRELOAD",
      "privilege-escalation"
    ],
    [
      "T1059.007",
      "JavaScript/JScript",
      "execution"
    ],
    [
      "T1546.005",
      "Trap",
      "privilege-escalation"
    ],
    [
      "T1559.001",
      "Component Object Model",
      "execution"
    ],
    [
      "T1574.012",
      "COR_PROFILER",
      "persistence"
    ],
    [
      "T1070.002",
      "Clear Linux or Mac System Logs",
      "defense-evasion"
    ],
    [
      "T1546.011",
      "Application Shimming",
      "persistence"
    ],
    [
      "T1098.002",
      "Exchange Email Delegate Permissions",
      "persistence"
    ],
    [
      "T1134.005",
      "SID-History Injection",
      "defense-evasion"
    ],
    [
      "T1037.005",
      "Startup Items",
      "persistence"
    ],
    [
      "T1218.007",
      "Msiexec",
      "defense-evasion"
    ],
    [
      "T1550.004",
      "Web Session Cookie",
      "lateral-movement"
    ],
    [
      "T1568.002",
      "Domain Generation Algorithms",
      "command-and-control"
    ],
    [
      "T1555.001",
      "Keychain",
      "credential-access"
    ],
    [
      "T1114.002",
      "Remote Email Collection",
      "collection"
    ],
    [
      "T1547.001",
      "Registry Run Keys / Startup Folder",
      "persistence"
    ],
    [
      "T1546.004",
      ".bash_profile and .bashrc",
      "privilege-escalation"
    ],
    [
      "T1553.003",
      "SIP and Trust Provider Hijacking",
      "defense-evasion"
    ],
    [
      "T1556.002",
      "Password Filter DLL",
      "defense-evasion"
    ],
    [
      "T1556.003",
      "Pluggable Authentication Modules",
      "credential-access"
    ],
    [
      "T1055.013",
      "Process Doppelgänging",
      "defense-evasion"
    ],
    [
      "T1560.001",
      "Archive via Utility",
      "collection"
    ],
    [
      "T1074.001",
      "Local Data Staging",
      "collection"
    ],
    [
      "T1070.006",
      "Timestomp",
      "defense-evasion"
    ],
    [
      "T1546.004",
      ".bash_profile and .bashrc",
      "persistence"
    ],
    [
      "T1543.003",
      "Windows Service",
      "privilege-escalation"
    ],
    [
      "T1564.001",
      "Hidden Files and Directories",
      "defense-evasion"
    ],
    [
      "T1574.008",
      "Path Interception by Search Order Hijacking",
      "defense-evasion"
    ],
    [
      "T1574.002",
      "DLL Side-Loading",
      "privilege-escalation"
    ],
    [
      "T1036.001",
      "Invalid Code Signature",
      "defense-evasion"
    ],
    [
      "T1132.002",
      "Non-Standard Encoding",
      "command-and-control"
    ],
    [
      "T1218.009",
      "Regsvcs/Regasm",
      "defense-evasion"
    ],
    [
      "T1560.002",
      "Archive via Library",
      "collection"
    ],
    [
      "T1003.002",
      "Security Account Manager",
      "credential-access"
    ],
    [
      "T1498.001",
      "Direct Network Flood",
      "impact"
    ],
    [
      "T1055.014",
      "VDSO Hijacking",
      "defense-evasion"
    ],
    [
      "T1565.003",
      "Runtime Data Manipulation",
      "impact"
    ],
    [
      "T1078.001",
      "Default Accounts",
      "defense-evasion"
    ],
    [
      "T1090.003",
      "Multi-hop Proxy",
      "command-and-control"
    ],
    [
      "T1542.002",
      "Component Firmware",
      "persistence"
    ],
    [
      "T1546.012",
      "Image File Execution Options Injection",
      "privilege-escalation"
    ],
    [
      "T1110.004",
      "Credential Stuffing",
      "credential-access"
    ],
    [
      "T1553.004",
      "Install Root Certificate",
      "defense-evasion"
    ],
    [
      "T1056.004",
      "Credential API Hooking",
      "credential-access"
    ],
    [
      "T1037.002",
      "Logon Script (Mac)",
      "persistence"
    ],
    [
      "T1027.005",
      "Indicator Removal from Tools",
      "defense-evasion"
    ],
    [
      "T1547.003",
      "Time Providers",
      "privilege-escalation"
    ],
    [
      "T1053.004",
      "Launchd",
      "persistence"
    ],
    [
      "T1134.001",
      "Token Impersonation/Theft",
      "privilege-escalation"
    ],
    [
      "T1137.001",
      "Office Template Macros",
      "persistence"
    ],
    [
      "T1543.001",
      "Launch Agent",
      "persistence"
    ],
    [
      "T1218.003",
      "CMSTP",
      "defense-evasion"
    ],
    [
      "T1134.003",
      "Make and Impersonate Token",
      "privilege-escalation"
    ],
    [
      "T1090.002",
      "External Proxy",
      "command-and-control"
    ],
    [
      "T1560.003",
      "Archive via Custom Method",
      "collection"
    ],
    [
      "T1546.009",
      "AppCert DLLs",
      "privilege-escalation"
    ],
    [
      "T1056.003",
      "Web Portal Capture",
      "collection"
    ],
    [
      "T1574.006",
      "LD_PRELOAD",
      "persistence"
    ],
    [
      "T1134.003",
      "Make and Impersonate Token",
      "defense-evasion"
    ],
    [
      "T1574.011",
      "Services Registry Permissions Weakness",
      "defense-evasion"
    ],
    [
      "T1566.003",
      "Spearphishing via Service",
      "initial-access"
    ],
    [
      "T1548.002",
      "Bypass User Access Control",
      "defense-evasion"
    ],
    [
      "T1553.002",
      "Code Signing",
      "defense-evasion"
    ],
    [
      "T1218.002",
      "Control Panel",
      "defense-evasion"
    ],
    [
      "T1552.001",
      "Credentials In Files",
      "credential-access"
    ],
    [
      "T1053.003",
      "Cron",
      "execution"
    ],
    [
      "T1568.001",
      "Fast Flux DNS",
      "command-and-control"
    ],
    [
      "T1078.003",
      "Local Accounts",
      "persistence"
    ],
    [
      "T1114.001",
      "Local Email Collection",
      "collection"
    ],
    [
      "T1574.008",
      "Path Interception by Search Order Hijacking",
      "privilege-escalation"
    ],
    [
      "T1567.002",
      "Exfiltration to Cloud Storage",
      "exfiltration"
    ],
    [
      "T1563.002",
      "RDP Hijacking",
      "lateral-movement"
    ],
    [
      "T1213.002",
      "Sharepoint",
      "collection"
    ],
    [
      "T1566.001",
      "Spearphishing Attachment",
      "initial-access"
    ],
    [
      "T1543.002",
      "Systemd Service",
      "persistence"
    ],
    [
      "T1543.001",
      "Launch Agent",
      "privilege-escalation"
    ],
    [
      "T1546.013",
      "PowerShell Profile",
      "privilege-escalation"
    ],
    [
      "T1552.004",
      "Private Keys",
      "credential-access"
    ],
    [
      "T1543.002",
      "Systemd Service",
      "privilege-escalation"
    ],
    [
      "T1134.001",
      "Token Impersonation/Theft",
      "defense-evasion"
    ],
    [
      "T1110.002",
      "Password Cracking",
      "credential-access"
    ],
    [
      "T1491.001",
      "Internal Defacement",
      "impact"
    ],
    [
      "T1037.001",
      "Logon Script (Windows)",
      "persistence"
    ],
    [
      "T1053.005",
      "Scheduled Task",
      "execution"
    ],
    [
      "T1574.004",
      "Dylib Hijacking",
      "defense-evasion"
    ],
    [
      "T1078.002",
      "Domain Accounts",
      "defense-evasion"
    ],
    [
      "T1546.015",
      "Component Object Model Hijacking",
      "privilege-escalation"
    ],
    [
      "T1547.006",
      "Kernel Modules and Extensions",
      "persistence"
    ],
    [
      "T1574.009",
      "Path Interception by Unquoted Path",
      "defense-evasion"
    ],
    [
      "T1195.003",
      "Compromise Hardware Supply Chain",
      "initial-access"
    ],
    [
      "T1546.013",
      "PowerShell Profile",
      "persistence"
    ],
    [
      "T1574.007",
      "Path Interception by PATH Environment Variable",
      "privilege-escalation"
    ],
    [
      "T1036.003",
      "Rename System Utilities",
      "defense-evasion"
    ],
    [
      "T1574.010",
      "Services File Permissions Weakness",
      "defense-evasion"
    ],
    [
      "T1090.004",
      "Domain Fronting",
      "command-and-control"
    ],
    [
      "T1055.014",
      "VDSO Hijacking",
      "privilege-escalation"
    ],
    [
      "T1498.002",
      "Reflection Amplification",
      "impact"
    ],
    [
      "T1574.001",
      "DLL Search Order Hijacking",
      "privilege-escalation"
    ],
    [
      "T1491.002",
      "External Defacement",
      "impact"
    ],
    [
      "T1546.012",
      "Image File Execution Options Injection",
      "persistence"
    ],
    [
      "T1001.001",
      "Junk Data",
      "command-and-control"
    ],
    [
      "T1218.005",
      "Mshta",
      "defense-evasion"
    ],
    [
      "T1574.010",
      "Services File Permissions Weakness",
      "privilege-escalation"
    ],
    [
      "T1205.001",
      "Port Knocking",
      "command-and-control"
    ],
    [
      "T1110.001",
      "Password Guessing",
      "credential-access"
    ],
    [
      "T1003.001",
      "LSASS Memory",
      "credential-access"
    ],
    [
      "T1055.009",
      "Proc Memory",
      "privilege-escalation"
    ],
    [
      "T1568.003",
      "DNS Calculation",
      "command-and-control"
    ],
    [
      "T1216.001",
      "PubPrn",
      "defense-evasion"
    ],
    [
      "T1574.010",
      "Services File Permissions Weakness",
      "persistence"
    ],
    [
      "T1548.004",
      "Elevated Execution with Prompt",
      "defense-evasion"
    ],
    [
      "T1218.008",
      "Odbcconf",
      "defense-evasion"
    ],
    [
      "T1548.001",
      "Setuid and Setgid",
      "privilege-escalation"
    ],
    [
      "T1574.004",
      "Dylib Hijacking",
      "privilege-escalation"
    ],
    [
      "T1218.004",
      "InstallUtil",
      "defense-evasion"
    ],
    [
      "T1078.003",
      "Local Accounts",
      "initial-access"
    ],
    [
      "T1071.003",
      "Mail Protocols",
      "command-and-control"
    ],
    [
      "T1574.011",
      "Services Registry Permissions Weakness",
      "privilege-escalation"
    ],
    [
      "T1550.004",
      "Web Session Cookie",
      "defense-evasion"
    ],
    [
      "T1053.004",
      "Launchd",
      "execution"
    ],
    [
      "T1078.004",
      "Cloud Accounts",
      "persistence"
    ],
    [
      "T1562.004",
      "Disable or Modify System Firewall",
      "defense-evasion"
    ],
    [
      "T1102.001",
      "Dead Drop Resolver",
      "command-and-control"
    ],
    [
      "T1036.006",
      "Space after Filename",
      "defense-evasion"
    ],
    [
      "T1574.002",
      "DLL Side-Loading",
      "defense-evasion"
    ],
    [
      "T1574.005",
      "Executable Installer File Permissions Weakness",
      "persistence"
    ],
    [
      "T1546.001",
      "Change Default File Association",
      "privilege-escalation"
    ],
    [
      "T1087.001",
      "Local Account",
      "discovery"
    ],
    [
      "T1053.002",
      "At (Windows)",
      "execution"
    ],
    [
      "T1542.003",
      "Bootkit",
      "persistence"
    ],
    [
      "T1137.005",
      "Outlook Rules",
      "persistence"
    ],
    [
      "T1053.005",
      "Scheduled Task",
      "privilege-escalation"
    ],
    [
      "T1569.002",
      "Service Execution",
      "execution"
    ],
    [
      "T1078.003",
      "Local Accounts",
      "privilege-escalation"
    ],
    [
      "T1102.002",
      "Bidirectional Communication",
      "command-and-control"
    ],
    [
      "T1546.014",
      "Emond",
      "persistence"
    ],
    [
      "T1562.006",
      "Indicator Blocking",
      "defense-evasion"
    ],
    [
      "T1110.003",
      "Password Spraying",
      "credential-access"
    ],
    [
      "T1037.004",
      "Rc.common",
      "persistence"
    ],
    [
      "T1027.002",
      "Software Packing",
      "defense-evasion"
    ],
    [
      "T1059.004",
      "Unix Shell",
      "execution"
    ],
    [
      "T1552.003",
      "Bash History",
      "credential-access"
    ],
    [
      "T1021.006",
      "Windows Remote Management",
      "lateral-movement"
    ],
    [
      "T1137.002",
      "Office Test",
      "persistence"
    ],
    [
      "T1137.003",
      "Outlook Forms",
      "persistence"
    ],
    [
      "T1055.013",
      "Process Doppelgänging",
      "privilege-escalation"
    ],
    [
      "T1574.009",
      "Path Interception by Unquoted Path",
      "persistence"
    ],
    [
      "T1059.001",
      "PowerShell",
      "execution"
    ],
    [
      "T1546.003",
      "Windows Management Instrumentation Event Subscription",
      "privilege-escalation"
    ],
    [
      "T1547.005",
      "Security Support Provider",
      "persistence"
    ],
    [
      "T1078.004",
      "Cloud Accounts",
      "defense-evasion"
    ],
    [
      "T1547.007",
      "Re-opened Applications",
      "privilege-escalation"
    ],
    [
      "T1574.001",
      "DLL Search Order Hijacking",
      "defense-evasion"
    ],
    [
      "T1048.001",
      "Exfiltration Over Symmetric Encrypted Non-C2 Protocol",
      "exfiltration"
    ],
    [
      "T1056.001",
      "Keylogging",
      "collection"
    ],
    [
      "T1069.002",
      "Domain Groups",
      "discovery"
    ],
    [
      "T1557.001",
      "LLMNR/NBT-NS Poisoning and SMB Relay",
      "credential-access"
    ],
    [
      "T1557.001",
      "LLMNR/NBT-NS Poisoning and SMB Relay",
      "collection"
    ],
    [
      "T1505.001",
      "SQL Stored Procedures",
      "persistence"
    ],
    [
      "T1546.002",
      "Screensaver",
      "persistence"
    ],
    [
      "T1547.008",
      "LSASS Driver",
      "persistence"
    ],
    [
      "T1087.004",
      "Cloud Account",
      "discovery"
    ],
    [
      "T1114.003",
      "Email Forwarding Rule",
      "collection"
    ],
    [
      "T1547.010",
      "Port Monitors",
      "privilege-escalation"
    ],
    [
      "T1546.002",
      "Screensaver",
      "privilege-escalation"
    ],
    [
      "T1555.002",
      "Securityd Memory",
      "credential-access"
    ],
    [
      "T1078.002",
      "Domain Accounts",
      "initial-access"
    ],
    [
      "T1087.003",
      "Email Account",
      "discovery"
    ],
    [
      "T1574.005",
      "Executable Installer File Permissions Weakness",
      "privilege-escalation"
    ],
    [
      "T1574.006",
      "LD_PRELOAD",
      "defense-evasion"
    ],
    [
      "T1204.001",
      "Malicious Link",
      "execution"
    ],
    [
      "T1574.007",
      "Path Interception by PATH Environment Variable",
      "persistence"
    ],
    [
      "T1053.001",
      "At (Linux)",
      "persistence"
    ],
    [
      "T1055.003",
      "Thread Execution Hijacking",
      "privilege-escalation"
    ],
    [
      "T1565.002",
      "Transmitted Data Manipulation",
      "impact"
    ],
    [
      "T1547.004",
      "Winlogon Helper DLL",
      "privilege-escalation"
    ],
    [
      "T1218.001",
      "Compiled HTML File",
      "defense-evasion"
    ],
    [
      "T1567.001",
      "Exfiltration to Code Repository",
      "exfiltration"
    ],
    [
      "T1546.006",
      "LC_LOAD_DYLIB Addition",
      "privilege-escalation"
    ],
    [
      "T1552.006",
      "Group Policy Preferences",
      "credential-access"
    ],
    [
      "T1574.009",
      "Path Interception by Unquoted Path",
      "privilege-escalation"
    ],
    [
      "T1053.003",
      "Cron",
      "privilege-escalation"
    ],
    [
      "T1546.005",
      "Trap",
      "persistence"
    ],
    [
      "T1573.002",
      "Asymmetric Cryptography",
      "command-and-control"
    ],
    [
      "T1546.006",
      "LC_LOAD_DYLIB Addition",
      "persistence"
    ],
    [
      "T1543.004",
      "Launch Daemon",
      "persistence"
    ],
    [
      "T1001.002",
      "Steganography",
      "command-and-control"
    ],
    [
      "T1037.004",
      "Rc.common",
      "privilege-escalation"
    ],
    [
      "T1059.002",
      "AppleScript",
      "execution"
    ],
    [
      "T1021.004",
      "SSH",
      "lateral-movement"
    ],
    [
      "T1078.002",
      "Domain Accounts",
      "privilege-escalation"
    ],
    [
      "T1480.001",
      "Environmental Keying",
      "defense-evasion"
    ],
    [
      "T1055.004",
      "Asynchronous Procedure Call",
      "privilege-escalation"
    ],
    [
      "T1069.001",
      "Local Groups",
      "discovery"
    ],
    [
      "T1055.011",
      "Extra Window Memory Injection",
      "privilege-escalation"
    ],
    [
      "T1547.011",
      "Plist Modification",
      "persistence"
    ],
    [
      "T1136.002",
      "Domain Account",
      "persistence"
    ],
    [
      "T1550.003",
      "Pass the Ticket",
      "lateral-movement"
    ],
    [
      "T1218.010",
      "Regsvr32",
      "defense-evasion"
    ],
    [
      "T1021.002",
      "SMB/Windows Admin Shares",
      "lateral-movement"
    ],
    [
      "T1003.007",
      "Proc Filesystem",
      "credential-access"
    ],
    [
      "T1542.001",
      "System Firmware",
      "persistence"
    ],
    [
      "T1127.001",
      "MSBuild",
      "defense-evasion"
    ],
    [
      "T1578.004",
      "Revert Cloud Instance",
      "defense-evasion"
    ],
    [
      "T1542.001",
      "System Firmware",
      "defense-evasion"
    ],
    [
      "T1497.002",
      "User Activity Based Checks",
      "discovery"
    ],
    [
      "T1566.002",
      "Spearphishing Link",
      "initial-access"
    ],
    [
      "T1011.001",
      "Exfiltration Over Bluetooth",
      "exfiltration"
    ],
    [
      "T1547.011",
      "Plist Modification",
      "privilege-escalation"
    ],
    [
      "T1136.001",
      "Local Account",
      "persistence"
    ],
    [
      "T1134.004",
      "Parent PID Spoofing",
      "privilege-escalation"
    ],
    [
      "T1027.001",
      "Binary Padding",
      "defense-evasion"
    ],
    [
      "T1074.002",
      "Remote Data Staging",
      "collection"
    ],
    [
      "T1497.001",
      "System Checks",
      "discovery"
    ],
    [
      "T1078.004",
      "Cloud Accounts",
      "privilege-escalation"
    ],
    [
      "T1053.001",
      "At (Linux)",
      "execution"
    ],
    [
      "T1055.004",
      "Asynchronous Procedure Call",
      "defense-evasion"
    ],
    [
      "T1001.003",
      "Protocol Impersonation",
      "command-and-control"
    ],
    [
      "T1102.003",
      "One-Way Communication",
      "command-and-control"
    ],
    [
      "T1546.008",
      "Accessibility Features",
      "persistence"
    ],
    [
      "T1542.002",
      "Component Firmware",
      "defense-evasion"
    ],
    [
      "T1574.002",
      "DLL Side-Loading",
      "persistence"
    ],
    [
      "T1561.001",
      "Disk Content Wipe",
      "impact"
    ],
    [
      "T1569.001",
      "Launchctl",
      "execution"
    ],
    [
      "T1565.001",
      "Stored Data Manipulation",
      "impact"
    ],
    [
      "T1556.001",
      "Domain Controller Authentication",
      "defense-evasion"
    ],
    [
      "T1070.004",
      "File Deletion",
      "defense-evasion"
    ],
    [
      "T1056.004",
      "Credential API Hooking",
      "collection"
    ],
    [
      "T1548.003",
      "Sudo and Sudo Caching",
      "privilege-escalation"
    ],
    [
      "T1574.007",
      "Path Interception by PATH Environment Variable",
      "defense-evasion"
    ],
    [
      "T1090.001",
      "Internal Proxy",
      "command-and-control"
    ],
    [
      "T1036.005",
      "Match Legitimate Name or Location",
      "defense-evasion"
    ],
    [
      "T1137.004",
      "Outlook Home Page",
      "persistence"
    ],
    [
      "T1550.003",
      "Pass the Ticket",
      "defense-evasion"
    ],
    [
      "T1036.002",
      "Right-to-Left Override",
      "defense-evasion"
    ],
    [
      "T1505.003",
      "Web Shell",
      "persistence"
    ],
    [
      "T1547.002",
      "Authentication Package",
      "persistence"
    ],
    [
      "T1559.002",
      "Dynamic Data Exchange",
      "execution"
    ],
    [
      "T1036.004",
      "Masquerade Task or Service",
      "defense-evasion"
    ],
    [
      "T1055.008",
      "Ptrace System Calls",
      "privilege-escalation"
    ],
    [
      "T1055.002",
      "Portable Executable Injection",
      "defense-evasion"
    ],
    [
      "T1003.004",
      "LSA Secrets",
      "credential-access"
    ],
    [
      "T1134.005",
      "SID-History Injection",
      "privilege-escalation"
    ],
    [
      "T1562.001",
      "Disable or Modify Tools",
      "defense-evasion"
    ],
    [
      "T1547.006",
      "Kernel Modules and Extensions",
      "privilege-escalation"
    ],
    [
      "T1558.002",
      "Silver Ticket",
      "credential-access"
    ],
    [
      "T1562.002",
      "Disable Windows Event Logging",
      "defense-evasion"
    ],
    [
      "T1550.001",
      "Application Access Token",
      "defense-evasion"
    ],
    [
      "T1078.003",
      "Local Accounts",
      "defense-evasion"
    ],
    [
      "T1053.002",
      "At (Windows)",
      "persistence"
    ],
    [
      "T1546.007",
      "Netsh Helper DLL",
      "persistence"
    ],
    [
      "T1546.008",
      "Accessibility Features",
      "privilege-escalation"
    ]
  ],
  "platforms": [
    "Linux",
    "macOS",
    "Windows"
  ],
  "datasources": [
    "Data loss prevention",
    "OAuth audit logs",
    "Kernel drivers",
    "Netflow/Enclave netflow",
    "Loaded DLLs",
    "Network intrusion detection system",
    "System calls",
    "Process monitoring",
    "Process use of network",
    "AWS CloudTrail logs",
    "Named Pipes",
    "Sensor health and status",
    "Process command-line parameters",
    "API monitoring",
    "Host network interface",
    "BIOS",
    "Office 365 trace logs",
    "Component firmware",
    "Disk forensics",
    "Services",
    "Office 365 account logs",
    "Network protocol analysis",
    "Third-party application logs",
    "Web application firewall logs",
    "SSL/TLS inspection",
    "VBR",
    "Access tokens",
    "Anti-virus",
    "Email gateway",
    "DLL monitoring",
    "User interface",
    "Asset management",
    "Browser extensions",
    "Mail server",
    "EFI",
    "Packet capture",
    "DNS records",
    "Windows event logs",
    "Digital certificate logs",
    "Binary file metadata",
    "Environment variable",
    "Azure activity logs",
    "File monitoring",
    "Web logs",
    "Stackdriver logs",
    "Application logs",
    "Windows Registry",
    "Detonation chamber",
    "Malware reverse engineering",
    "Windows Error Reporting",
    "WMI Objects",
    "Web proxy",
    "Authentication logs",
    "PowerShell logs",
    "Office 365 audit logs",
    "MBR",
    "GCP audit logs",
    "Network device logs"
  ],
  "revoked_techniques": [
    [
      "T1101",
      "Security Support Provider"
    ],
    [
      "T1146",
      "Clear Command History"
    ],
    [
      "T1117",
      "Regsvr32"
    ],
    [
      "T1043",
      "Commonly Used Port"
    ],
    [
      "T1148",
      "HISTCONTROL"
    ],
    [
      "T1161",
      "LC_LOAD_DYLIB Addition"
    ],
    [
      "T1109",
      "Component Firmware"
    ],
    [
      "T1159",
      "Launch Agent"
    ],
    [
      "T1168",
      "Local Job Scheduling"
    ],
    [
      "T1121",
      "Regsvcs/Regasm"
    ],
    [
      "T1158",
      "Hidden Files and Directories"
    ],
    [
      "T1223",
      "Compiled HTML File"
    ],
    [
      "T1096",
      "NTFS File Attributes"
    ],
    [
      "T1186",
      "Process Doppelgänging"
    ],
    [
      "T1032",
      "Standard Cryptographic Protocol"
    ],
    [
      "T1506",
      "Web Session Cookie"
    ],
    [
      "T1077",
      "Windows Admin Shares"
    ],
    [
      "T1107",
      "File Deletion"
    ],
    [
      "T1500",
      "Compile After Delivery"
    ],
    [
      "T1170",
      "Mshta"
    ],
    [
      "T1028",
      "Windows Remote Management"
    ],
    [
      "T1022",
      "Data Encrypted"
    ],
    [
      "T1139",
      "Bash History"
    ],
    [
      "T1031",
      "Modify Existing Service"
    ],
    [
      "T1089",
      "Disabling Security Tools"
    ],
    [
      "T1122",
      "Component Object Model Hijacking"
    ],
    [
      "T1075",
      "Pass the Hash"
    ],
    [
      "T1155",
      "AppleScript"
    ],
    [
      "T1160",
      "Launch Daemon"
    ],
    [
      "T1198",
      "SIP and Trust Provider Hijacking"
    ],
    [
      "T1054",
      "Indicator Blocking"
    ],
    [
      "T1180",
      "Screensaver"
    ],
    [
      "T1519",
      "Emond"
    ],
    [
      "T1502",
      "Parent PID Spoofing"
    ],
    [
      "T1157",
      "Dylib Hijacking"
    ],
    [
      "T1536",
      "Revert Cloud Instance"
    ],
    [
      "T1060",
      "Registry Run Keys / Startup Folder"
    ],
    [
      "T1099",
      "Timestomp"
    ],
    [
      "T1172",
      "Domain Fronting"
    ],
    [
      "T1165",
      "Startup Items"
    ],
    [
      "T1193",
      "Spearphishing Attachment"
    ],
    [
      "T1173",
      "Dynamic Data Exchange"
    ],
    [
      "T1208",
      "Kerberoasting"
    ],
    [
      "T1126",
      "Network Share Connection Removal"
    ],
    [
      "T1076",
      "Remote Desktop Protocol"
    ],
    [
      "T1058",
      "Service Registry Permissions Weakness"
    ],
    [
      "T1175",
      "Component Object Model and Distributed COM"
    ],
    [
      "T1150",
      "Plist Modification"
    ],
    [
      "T1178",
      "SID-History Injection"
    ],
    [
      "T1205",
      "Traffic Signaling"
    ],
    [
      "T1085",
      "Rundll32"
    ],
    [
      "T1184",
      "SSH Hijacking"
    ],
    [
      "T1009",
      "Binary Padding"
    ],
    [
      "T1023",
      "Shortcut Modification"
    ],
    [
      "T1494",
      "Runtime Data Manipulation"
    ],
    [
      "T1059",
      "Command and Scripting Interpreter"
    ],
    [
      "T1142",
      "Keychain"
    ],
    [
      "T1154",
      "Trap"
    ],
    [
      "T1128",
      "Netsh Helper DLL"
    ],
    [
      "T1483",
      "Domain Generation Algorithms"
    ],
    [
      "T1044",
      "File System Permissions Weakness"
    ],
    [
      "T1503",
      "Credentials from Web Browsers"
    ],
    [
      "T1504",
      "PowerShell Profile"
    ],
    [
      "T1145",
      "Private Keys"
    ],
    [
      "T1141",
      "Input Prompt"
    ],
    [
      "T1169",
      "Sudo"
    ],
    [
      "T1179",
      "Hooking"
    ],
    [
      "T1079",
      "Multilayer Encryption"
    ],
    [
      "T1487",
      "Disk Structure Wipe"
    ],
    [
      "T1042",
      "Change Default File Association"
    ],
    [
      "T1088",
      "Bypass User Account Control"
    ],
    [
      "T1024",
      "Custom Cryptographic Protocol"
    ],
    [
      "T1164",
      "Re-opened Applications"
    ],
    [
      "T1183",
      "Image File Execution Options Injection"
    ],
    [
      "T1097",
      "Pass the Ticket"
    ],
    [
      "T1522",
      "Cloud Instance Metadata API"
    ],
    [
      "T1004",
      "Winlogon Helper DLL"
    ],
    [
      "T1196",
      "Control Panel Items"
    ],
    [
      "T1493",
      "Transmitted Data Manipulation"
    ],
    [
      "T1081",
      "Credentials in Files"
    ],
    [
      "T1553",
      "Subvert Trust Controls"
    ],
    [
      "T1181",
      "Extra Window Memory Injection"
    ],
    [
      "T1050",
      "New Service"
    ],
    [
      "T1002",
      "Data Compressed"
    ],
    [
      "T1015",
      "Accessibility Features"
    ],
    [
      "T1492",
      "Stored Data Manipulation"
    ],
    [
      "T1152",
      "Launchctl"
    ],
    [
      "T1214",
      "Credentials in Registry"
    ],
    [
      "T1188",
      "Multi-hop Proxy"
    ],
    [
      "T1093",
      "Process Hollowing"
    ],
    [
      "T1019",
      "System Firmware"
    ],
    [
      "T1065",
      "Uncommonly Used Port"
    ],
    [
      "T1171",
      "LLMNR/NBT-NS Poisoning and Relay"
    ],
    [
      "T1013",
      "Port Monitors"
    ],
    [
      "T1064",
      "Scripting"
    ],
    [
      "T1514",
      "Elevated Execution with Prompt"
    ],
    [
      "T1063",
      "Security Software Discovery"
    ],
    [
      "T1045",
      "Software Packing"
    ],
    [
      "T1209",
      "Time Providers"
    ],
    [
      "T1147",
      "Hidden Users"
    ],
    [
      "T1501",
      "Systemd Service"
    ],
    [
      "T1144",
      "Gatekeeper Bypass"
    ],
    [
      "T1138",
      "Application Shimming"
    ],
    [
      "T1131",
      "Authentication Package"
    ],
    [
      "T1034",
      "Path Interception"
    ],
    [
      "T1192",
      "Spearphishing Link"
    ],
    [
      "T1162",
      "Login Item"
    ],
    [
      "T1488",
      "Disk Content Wipe"
    ],
    [
      "T1017",
      "Application Deployment Software"
    ],
    [
      "T1094",
      "Custom Command and Control Protocol"
    ],
    [
      "T1143",
      "Hidden Window"
    ],
    [
      "T1086",
      "PowerShell"
    ],
    [
      "T1182",
      "AppCert DLLs"
    ],
    [
      "T1103",
      "AppInit DLLs"
    ],
    [
      "T1100",
      "Web Shell"
    ],
    [
      "T1167",
      "Securityd Memory"
    ],
    [
      "T1166",
      "Setuid and Setgid"
    ],
    [
      "T1151",
      "Space after Filename"
    ],
    [
      "T1527",
      "Application Access Token"
    ],
    [
      "T1191",
      "CMSTP"
    ],
    [
      "T1118",
      "InstallUtil"
    ],
    [
      "T1130",
      "Install Root Certificate"
    ],
    [
      "T1084",
      "Windows Management Instrumentation Event Subscription"
    ],
    [
      "T1067",
      "Bootkit"
    ],
    [
      "T1174",
      "Password Filter DLL"
    ],
    [
      "T1215",
      "Kernel Modules and Extensions"
    ],
    [
      "T1206",
      "Sudo Caching"
    ],
    [
      "T1066",
      "Indicator Removal from Tools"
    ],
    [
      "T1177",
      "LSASS Driver"
    ],
    [
      "T1163",
      "Rc.common"
    ],
    [
      "T1038",
      "DLL Search Order Hijacking"
    ],
    [
      "T1194",
      "Spearphishing via Service"
    ],
    [
      "T1035",
      "Service Execution"
    ],
    [
      "T1116",
      "Code Signing"
    ],
    [
      "T1156",
      ".bash_profile and .bashrc"
    ],
    [
      "T1073",
      "DLL Side-Loading"
    ]
  ],
  "stats": {
    "count_revoked_techniques": 136,
    "count_active_techniques": 187,
    "count_active_subtechniques": 391,
    "count_malware_records": 351,
    "count_adversary_records": 109,
    "count_tools_records": 61,
    "count_platforms_records": 3,
    "count_tactics_records": 12,
    "count_datasources_records": 58
  }
}
```