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


**Note:  The stats key of this json will vary based on Mitre changing the
cti repo without our knowledge.


<br/>

```json
{
  "tactics": [
    "command-and-control",
    "collection",
    "initial-access",
    "defense-evasion",
    "exfiltration",
    "credential-access",
    "discovery",
    "privilege-escalation",
    "lateral-movement",
    "persistence",
    "impact",
    "execution"
  ],
  "platforms": [
    "Office 365",
    "SaaS",
    "AWS",
    "Azure AD",
    "Azure",
    "GCP",
    "Linux",
    "macOS",
    "Windows"
  ],
  "datasources": [
    "API monitoring",
    "AWS CloudTrail logs",
    "Access tokens",
    "Anti-virus",
    "Application logs",
    "Asset management",
    "Authentication logs",
    "Azure activity logs",
    "BIOS",
    "Binary file metadata",
    "Browser extensions",
    "Component firmware",
    "DLL monitoring",
    "DNS records",
    "Data loss prevention",
    "Detonation chamber",
    "Digital certificate logs",
    "Disk forensics",
    "EFI",
    "Email gateway",
    "Environment variable",
    "File monitoring",
    "GCP audit logs",
    "Host network interface",
    "Kernel drivers",
    "Loaded DLLs",
    "MBR",
    "Mail server",
    "Malware reverse engineering",
    "Named Pipes",
    "Netflow/Enclave netflow",
    "Network device logs",
    "Network intrusion detection system",
    "Network protocol analysis",
    "OAuth audit logs",
    "Office 365 account logs",
    "Office 365 audit logs",
    "Office 365 trace logs",
    "Packet capture",
    "PowerShell logs",
    "Process command-line parameters",
    "Process monitoring",
    "Process use of network",
    "SSL/TLS inspection",
    "Sensor health and status",
    "Services",
    "Stackdriver logs",
    "System calls",
    "Third-party application logs",
    "User interface",
    "VBR",
    "WMI Objects",
    "Web application firewall logs",
    "Web logs",
    "Web proxy",
    "Windows Error Reporting",
    "Windows Registry",
    "Windows event logs"
  ],
  "revoked_techniques": [
    [
      "T1162",
      "Login Item"
    ],
    [
      "T1158",
      "Hidden Files and Directories"
    ],
    [
      "T1175",
      "Component Object Model and Distributed COM"
    ],
    [
      "T1004",
      "Winlogon Helper DLL"
    ],
    [
      "T1519",
      "Emond"
    ],
    [
      "T1038",
      "DLL Search Order Hijacking"
    ],
    [
      "T1094",
      "Custom Command and Control Protocol"
    ],
    [
      "T1177",
      "LSASS Driver"
    ],
    [
      "T1188",
      "Multi-hop Proxy"
    ],
    [
      "T1506",
      "Web Session Cookie"
    ],
    [
      "T1130",
      "Install Root Certificate"
    ],
    [
      "T1214",
      "Credentials in Registry"
    ],
    [
      "T1492",
      "Stored Data Manipulation"
    ],
    [
      "T1035",
      "Service Execution"
    ],
    [
      "T1118",
      "InstallUtil"
    ],
    [
      "T1178",
      "SID-History Injection"
    ],
    [
      "T1150",
      "Plist Modification"
    ],
    [
      "T1009",
      "Binary Padding"
    ],
    [
      "T1171",
      "LLMNR/NBT-NS Poisoning and Relay"
    ],
    [
      "T1117",
      "Regsvr32"
    ],
    [
      "T1205",
      "Traffic Signaling"
    ],
    [
      "T1089",
      "Disabling Security Tools"
    ],
    [
      "T1022",
      "Data Encrypted"
    ],
    [
      "T1183",
      "Image File Execution Options Injection"
    ],
    [
      "T1103",
      "AppInit DLLs"
    ],
    [
      "T1138",
      "Application Shimming"
    ],
    [
      "T1223",
      "Compiled HTML File"
    ],
    [
      "T1144",
      "Gatekeeper Bypass"
    ],
    [
      "T1142",
      "Keychain"
    ],
    [
      "T1181",
      "Extra Window Memory Injection"
    ],
    [
      "T1143",
      "Hidden Window"
    ],
    [
      "T1155",
      "AppleScript"
    ],
    [
      "T1493",
      "Transmitted Data Manipulation"
    ],
    [
      "T1168",
      "Local Job Scheduling"
    ],
    [
      "T1054",
      "Indicator Blocking"
    ],
    [
      "T1032",
      "Standard Cryptographic Protocol"
    ],
    [
      "T1002",
      "Data Compressed"
    ],
    [
      "T1107",
      "File Deletion"
    ],
    [
      "T1174",
      "Password Filter DLL"
    ],
    [
      "T1079",
      "Multilayer Encryption"
    ],
    [
      "T1086",
      "PowerShell"
    ],
    [
      "T1173",
      "Dynamic Data Exchange"
    ],
    [
      "T1536",
      "Revert Cloud Instance"
    ],
    [
      "T1186",
      "Process Doppelgänging"
    ],
    [
      "T1164",
      "Re-opened Applications"
    ],
    [
      "T1483",
      "Domain Generation Algorithms"
    ],
    [
      "T1146",
      "Clear Command History"
    ],
    [
      "T1081",
      "Credentials in Files"
    ],
    [
      "T1487",
      "Disk Structure Wipe"
    ],
    [
      "T1017",
      "Application Deployment Software"
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
      "T1019",
      "System Firmware"
    ],
    [
      "T1045",
      "Software Packing"
    ],
    [
      "T1215",
      "Kernel Modules and Extensions"
    ],
    [
      "T1075",
      "Pass the Hash"
    ],
    [
      "T1043",
      "Commonly Used Port"
    ],
    [
      "T1066",
      "Indicator Removal from Tools"
    ],
    [
      "T1122",
      "Component Object Model Hijacking"
    ],
    [
      "T1163",
      "Rc.common"
    ],
    [
      "T1167",
      "Securityd Memory"
    ],
    [
      "T1191",
      "CMSTP"
    ],
    [
      "T1100",
      "Web Shell"
    ],
    [
      "T1504",
      "PowerShell Profile"
    ],
    [
      "T1060",
      "Registry Run Keys / Startup Folder"
    ],
    [
      "T1077",
      "Windows Admin Shares"
    ],
    [
      "T1028",
      "Windows Remote Management"
    ],
    [
      "T1169",
      "Sudo"
    ],
    [
      "T1067",
      "Bootkit"
    ],
    [
      "T1059",
      "Command and Scripting Interpreter"
    ],
    [
      "T1058",
      "Service Registry Permissions Weakness"
    ],
    [
      "T1044",
      "File System Permissions Weakness"
    ],
    [
      "T1151",
      "Space after Filename"
    ],
    [
      "T1015",
      "Accessibility Features"
    ],
    [
      "T1165",
      "Startup Items"
    ],
    [
      "T1502",
      "Parent PID Spoofing"
    ],
    [
      "T1109",
      "Component Firmware"
    ],
    [
      "T1023",
      "Shortcut Modification"
    ],
    [
      "T1131",
      "Authentication Package"
    ],
    [
      "T1161",
      "LC_LOAD_DYLIB Addition"
    ],
    [
      "T1184",
      "SSH Hijacking"
    ],
    [
      "T1192",
      "Spearphishing Link"
    ],
    [
      "T1553",
      "Subvert Trust Controls"
    ],
    [
      "T1031",
      "Modify Existing Service"
    ],
    [
      "T1097",
      "Pass the Ticket"
    ],
    [
      "T1065",
      "Uncommonly Used Port"
    ],
    [
      "T1501",
      "Systemd Service"
    ],
    [
      "T1093",
      "Process Hollowing"
    ],
    [
      "T1076",
      "Remote Desktop Protocol"
    ],
    [
      "T1042",
      "Change Default File Association"
    ],
    [
      "T1196",
      "Control Panel Items"
    ],
    [
      "T1073",
      "DLL Side-Loading"
    ],
    [
      "T1182",
      "AppCert DLLs"
    ],
    [
      "T1180",
      "Screensaver"
    ],
    [
      "T1206",
      "Sudo Caching"
    ],
    [
      "T1148",
      "HISTCONTROL"
    ],
    [
      "T1193",
      "Spearphishing Attachment"
    ],
    [
      "T1064",
      "Scripting"
    ],
    [
      "T1088",
      "Bypass User Account Control"
    ],
    [
      "T1013",
      "Port Monitors"
    ],
    [
      "T1096",
      "NTFS File Attributes"
    ],
    [
      "T1034",
      "Path Interception"
    ],
    [
      "T1198",
      "SIP and Trust Provider Hijacking"
    ],
    [
      "T1141",
      "Input Prompt"
    ],
    [
      "T1160",
      "Launch Daemon"
    ],
    [
      "T1194",
      "Spearphishing via Service"
    ],
    [
      "T1085",
      "Rundll32"
    ],
    [
      "T1494",
      "Runtime Data Manipulation"
    ],
    [
      "T1152",
      "Launchctl"
    ],
    [
      "T1157",
      "Dylib Hijacking"
    ],
    [
      "T1514",
      "Elevated Execution with Prompt"
    ],
    [
      "T1170",
      "Mshta"
    ],
    [
      "T1050",
      "New Service"
    ],
    [
      "T1145",
      "Private Keys"
    ],
    [
      "T1063",
      "Security Software Discovery"
    ],
    [
      "T1139",
      "Bash History"
    ],
    [
      "T1488",
      "Disk Content Wipe"
    ],
    [
      "T1156",
      ".bash_profile and .bashrc"
    ],
    [
      "T1522",
      "Cloud Instance Metadata API"
    ],
    [
      "T1503",
      "Credentials from Web Browsers"
    ],
    [
      "T1024",
      "Custom Cryptographic Protocol"
    ],
    [
      "T1128",
      "Netsh Helper DLL"
    ],
    [
      "T1179",
      "Hooking"
    ],
    [
      "T1166",
      "Setuid and Setgid"
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
      "T1147",
      "Hidden Users"
    ],
    [
      "T1101",
      "Security Support Provider"
    ],
    [
      "T1084",
      "Windows Management Instrumentation Event Subscription"
    ],
    [
      "T1159",
      "Launch Agent"
    ],
    [
      "T1209",
      "Time Providers"
    ],
    [
      "T1154",
      "Trap"
    ],
    [
      "T1527",
      "Application Access Token"
    ],
    [
      "T1500",
      "Compile After Delivery"
    ],
    [
      "T1116",
      "Code Signing"
    ],
    [
      "T1121",
      "Regsvcs/Regasm"
    ]
  ],
  "breakdown_techniques": {
    "count": 621,
    "platforms": [
      {
        "platform": "Linux",
        "tid": "T1548",
        "technique": "Abuse Elevation Control Mechanism",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1548",
        "technique": "Abuse Elevation Control Mechanism",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1548",
        "technique": "Abuse Elevation Control Mechanism",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1548",
        "technique": "Abuse Elevation Control Mechanism",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1548",
        "technique": "Abuse Elevation Control Mechanism",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1548",
        "technique": "Abuse Elevation Control Mechanism",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1134",
        "technique": "Access Token Manipulation",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1134",
        "technique": "Access Token Manipulation",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1531",
        "technique": "Account Access Removal",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1531",
        "technique": "Account Access Removal",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1531",
        "technique": "Account Access Removal",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1087",
        "technique": "Account Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1087",
        "technique": "Account Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1087",
        "technique": "Account Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Office 365",
        "tid": "T1087",
        "technique": "Account Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure AD",
        "tid": "T1087",
        "technique": "Account Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1087",
        "technique": "Account Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1087",
        "technique": "Account Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1087",
        "technique": "Account Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "SaaS",
        "tid": "T1087",
        "technique": "Account Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1098",
        "technique": "Account Manipulation",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1098",
        "technique": "Account Manipulation",
        "tactic": "persistence"
      },
      {
        "platform": "Azure",
        "tid": "T1098",
        "technique": "Account Manipulation",
        "tactic": "persistence"
      },
      {
        "platform": "GCP",
        "tid": "T1098",
        "technique": "Account Manipulation",
        "tactic": "persistence"
      },
      {
        "platform": "Azure AD",
        "tid": "T1098",
        "technique": "Account Manipulation",
        "tactic": "persistence"
      },
      {
        "platform": "AWS",
        "tid": "T1098",
        "technique": "Account Manipulation",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1098",
        "technique": "Account Manipulation",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1098",
        "technique": "Account Manipulation",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1071",
        "technique": "Application Layer Protocol",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1071",
        "technique": "Application Layer Protocol",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1071",
        "technique": "Application Layer Protocol",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1010",
        "technique": "Application Window Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1010",
        "technique": "Application Window Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1560",
        "technique": "Archive Collected Data",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1560",
        "technique": "Archive Collected Data",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1560",
        "technique": "Archive Collected Data",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1123",
        "technique": "Audio Capture",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1123",
        "technique": "Audio Capture",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1123",
        "technique": "Audio Capture",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1119",
        "technique": "Automated Collection",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1119",
        "technique": "Automated Collection",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1119",
        "technique": "Automated Collection",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1020",
        "technique": "Automated Exfiltration",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1020",
        "technique": "Automated Exfiltration",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1020",
        "technique": "Automated Exfiltration",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1197",
        "technique": "BITS Jobs",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1197",
        "technique": "BITS Jobs",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1547",
        "technique": "Boot or Logon Autostart Execution",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1547",
        "technique": "Boot or Logon Autostart Execution",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1547",
        "technique": "Boot or Logon Autostart Execution",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1547",
        "technique": "Boot or Logon Autostart Execution",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1547",
        "technique": "Boot or Logon Autostart Execution",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1547",
        "technique": "Boot or Logon Autostart Execution",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1037",
        "technique": "Boot or Logon Initialization Scripts",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1037",
        "technique": "Boot or Logon Initialization Scripts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1037",
        "technique": "Boot or Logon Initialization Scripts",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1037",
        "technique": "Boot or Logon Initialization Scripts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1217",
        "technique": "Browser Bookmark Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1217",
        "technique": "Browser Bookmark Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1217",
        "technique": "Browser Bookmark Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1176",
        "technique": "Browser Extensions",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1176",
        "technique": "Browser Extensions",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1176",
        "technique": "Browser Extensions",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1110",
        "technique": "Brute Force",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1110",
        "technique": "Brute Force",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1110",
        "technique": "Brute Force",
        "tactic": "credential-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1110",
        "technique": "Brute Force",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure AD",
        "tid": "T1110",
        "technique": "Brute Force",
        "tactic": "credential-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1110",
        "technique": "Brute Force",
        "tactic": "credential-access"
      },
      {
        "platform": "GCP",
        "tid": "T1110",
        "technique": "Brute Force",
        "tactic": "credential-access"
      },
      {
        "platform": "AWS",
        "tid": "T1110",
        "technique": "Brute Force",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure",
        "tid": "T1110",
        "technique": "Brute Force",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1115",
        "technique": "Clipboard Data",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1115",
        "technique": "Clipboard Data",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1115",
        "technique": "Clipboard Data",
        "tactic": "collection"
      },
      {
        "platform": "AWS",
        "tid": "T1538",
        "technique": "Cloud Service Dashboard",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1538",
        "technique": "Cloud Service Dashboard",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1538",
        "technique": "Cloud Service Dashboard",
        "tactic": "discovery"
      },
      {
        "platform": "Azure AD",
        "tid": "T1538",
        "technique": "Cloud Service Dashboard",
        "tactic": "discovery"
      },
      {
        "platform": "Office 365",
        "tid": "T1538",
        "technique": "Cloud Service Dashboard",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1526",
        "technique": "Cloud Service Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1526",
        "technique": "Cloud Service Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1526",
        "technique": "Cloud Service Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure AD",
        "tid": "T1526",
        "technique": "Cloud Service Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Office 365",
        "tid": "T1526",
        "technique": "Cloud Service Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "SaaS",
        "tid": "T1526",
        "technique": "Cloud Service Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1092",
        "technique": "Communication Through Removable Media",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1092",
        "technique": "Communication Through Removable Media",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1092",
        "technique": "Communication Through Removable Media",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1554",
        "technique": "Compromise Client Software Binary",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1554",
        "technique": "Compromise Client Software Binary",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1554",
        "technique": "Compromise Client Software Binary",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1136",
        "technique": "Create Account",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1136",
        "technique": "Create Account",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1136",
        "technique": "Create Account",
        "tactic": "persistence"
      },
      {
        "platform": "AWS",
        "tid": "T1136",
        "technique": "Create Account",
        "tactic": "persistence"
      },
      {
        "platform": "GCP",
        "tid": "T1136",
        "technique": "Create Account",
        "tactic": "persistence"
      },
      {
        "platform": "Azure AD",
        "tid": "T1136",
        "technique": "Create Account",
        "tactic": "persistence"
      },
      {
        "platform": "Azure",
        "tid": "T1136",
        "technique": "Create Account",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1136",
        "technique": "Create Account",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1543",
        "technique": "Create or Modify System Process",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1543",
        "technique": "Create or Modify System Process",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1543",
        "technique": "Create or Modify System Process",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1543",
        "technique": "Create or Modify System Process",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1543",
        "technique": "Create or Modify System Process",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1543",
        "technique": "Create or Modify System Process",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1555",
        "technique": "Credentials from Password Stores",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1555",
        "technique": "Credentials from Password Stores",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1555",
        "technique": "Credentials from Password Stores",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1485",
        "technique": "Data Destruction",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1485",
        "technique": "Data Destruction",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1485",
        "technique": "Data Destruction",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1132",
        "technique": "Data Encoding",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1132",
        "technique": "Data Encoding",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1132",
        "technique": "Data Encoding",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1486",
        "technique": "Data Encrypted for Impact",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1486",
        "technique": "Data Encrypted for Impact",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1486",
        "technique": "Data Encrypted for Impact",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1565",
        "technique": "Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1565",
        "technique": "Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1565",
        "technique": "Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1001",
        "technique": "Data Obfuscation",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1001",
        "technique": "Data Obfuscation",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1001",
        "technique": "Data Obfuscation",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1074",
        "technique": "Data Staged",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1074",
        "technique": "Data Staged",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1074",
        "technique": "Data Staged",
        "tactic": "collection"
      },
      {
        "platform": "AWS",
        "tid": "T1074",
        "technique": "Data Staged",
        "tactic": "collection"
      },
      {
        "platform": "GCP",
        "tid": "T1074",
        "technique": "Data Staged",
        "tactic": "collection"
      },
      {
        "platform": "Azure",
        "tid": "T1074",
        "technique": "Data Staged",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1030",
        "technique": "Data Transfer Size Limits",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1030",
        "technique": "Data Transfer Size Limits",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1030",
        "technique": "Data Transfer Size Limits",
        "tactic": "exfiltration"
      },
      {
        "platform": "AWS",
        "tid": "T1530",
        "technique": "Data from Cloud Storage Object",
        "tactic": "collection"
      },
      {
        "platform": "GCP",
        "tid": "T1530",
        "technique": "Data from Cloud Storage Object",
        "tactic": "collection"
      },
      {
        "platform": "Azure",
        "tid": "T1530",
        "technique": "Data from Cloud Storage Object",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1213",
        "technique": "Data from Information Repositories",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1213",
        "technique": "Data from Information Repositories",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1213",
        "technique": "Data from Information Repositories",
        "tactic": "collection"
      },
      {
        "platform": "SaaS",
        "tid": "T1213",
        "technique": "Data from Information Repositories",
        "tactic": "collection"
      },
      {
        "platform": "AWS",
        "tid": "T1213",
        "technique": "Data from Information Repositories",
        "tactic": "collection"
      },
      {
        "platform": "GCP",
        "tid": "T1213",
        "technique": "Data from Information Repositories",
        "tactic": "collection"
      },
      {
        "platform": "Azure",
        "tid": "T1213",
        "technique": "Data from Information Repositories",
        "tactic": "collection"
      },
      {
        "platform": "Office 365",
        "tid": "T1213",
        "technique": "Data from Information Repositories",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1005",
        "technique": "Data from Local System",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1005",
        "technique": "Data from Local System",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1005",
        "technique": "Data from Local System",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1039",
        "technique": "Data from Network Shared Drive",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1039",
        "technique": "Data from Network Shared Drive",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1039",
        "technique": "Data from Network Shared Drive",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1025",
        "technique": "Data from Removable Media",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1025",
        "technique": "Data from Removable Media",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1025",
        "technique": "Data from Removable Media",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1491",
        "technique": "Defacement",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1491",
        "technique": "Defacement",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1491",
        "technique": "Defacement",
        "tactic": "impact"
      },
      {
        "platform": "AWS",
        "tid": "T1491",
        "technique": "Defacement",
        "tactic": "impact"
      },
      {
        "platform": "GCP",
        "tid": "T1491",
        "technique": "Defacement",
        "tactic": "impact"
      },
      {
        "platform": "Azure",
        "tid": "T1491",
        "technique": "Defacement",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1140",
        "technique": "Deobfuscate/Decode Files or Information",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1006",
        "technique": "Direct Volume Access",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1561",
        "technique": "Disk Wipe",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1561",
        "technique": "Disk Wipe",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1561",
        "technique": "Disk Wipe",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1482",
        "technique": "Domain Trust Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1189",
        "technique": "Drive-by Compromise",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1189",
        "technique": "Drive-by Compromise",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1189",
        "technique": "Drive-by Compromise",
        "tactic": "initial-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1189",
        "technique": "Drive-by Compromise",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1568",
        "technique": "Dynamic Resolution",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1568",
        "technique": "Dynamic Resolution",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1568",
        "technique": "Dynamic Resolution",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1114",
        "technique": "Email Collection",
        "tactic": "collection"
      },
      {
        "platform": "Office 365",
        "tid": "T1114",
        "technique": "Email Collection",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1573",
        "technique": "Encrypted Channel",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1573",
        "technique": "Encrypted Channel",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1573",
        "technique": "Encrypted Channel",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1499",
        "technique": "Endpoint Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1499",
        "technique": "Endpoint Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1499",
        "technique": "Endpoint Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "AWS",
        "tid": "T1499",
        "technique": "Endpoint Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "GCP",
        "tid": "T1499",
        "technique": "Endpoint Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "Azure",
        "tid": "T1499",
        "technique": "Endpoint Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "Office 365",
        "tid": "T1499",
        "technique": "Endpoint Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "Azure AD",
        "tid": "T1499",
        "technique": "Endpoint Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "SaaS",
        "tid": "T1499",
        "technique": "Endpoint Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1546",
        "technique": "Event Triggered Execution",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1546",
        "technique": "Event Triggered Execution",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1546",
        "technique": "Event Triggered Execution",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1546",
        "technique": "Event Triggered Execution",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1546",
        "technique": "Event Triggered Execution",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546",
        "technique": "Event Triggered Execution",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1480",
        "technique": "Execution Guardrails",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1480",
        "technique": "Execution Guardrails",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1480",
        "technique": "Execution Guardrails",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1048",
        "technique": "Exfiltration Over Alternative Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1048",
        "technique": "Exfiltration Over Alternative Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1048",
        "technique": "Exfiltration Over Alternative Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1041",
        "technique": "Exfiltration Over C2 Channel",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1041",
        "technique": "Exfiltration Over C2 Channel",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1041",
        "technique": "Exfiltration Over C2 Channel",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1011",
        "technique": "Exfiltration Over Other Network Medium",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1011",
        "technique": "Exfiltration Over Other Network Medium",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1011",
        "technique": "Exfiltration Over Other Network Medium",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1052",
        "technique": "Exfiltration Over Physical Medium",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1052",
        "technique": "Exfiltration Over Physical Medium",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1052",
        "technique": "Exfiltration Over Physical Medium",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1567",
        "technique": "Exfiltration Over Web Service",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1567",
        "technique": "Exfiltration Over Web Service",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1567",
        "technique": "Exfiltration Over Web Service",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1190",
        "technique": "Exploit Public-Facing Application",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1190",
        "technique": "Exploit Public-Facing Application",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1190",
        "technique": "Exploit Public-Facing Application",
        "tactic": "initial-access"
      },
      {
        "platform": "AWS",
        "tid": "T1190",
        "technique": "Exploit Public-Facing Application",
        "tactic": "initial-access"
      },
      {
        "platform": "GCP",
        "tid": "T1190",
        "technique": "Exploit Public-Facing Application",
        "tactic": "initial-access"
      },
      {
        "platform": "Azure",
        "tid": "T1190",
        "technique": "Exploit Public-Facing Application",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1203",
        "technique": "Exploitation for Client Execution",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1203",
        "technique": "Exploitation for Client Execution",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1203",
        "technique": "Exploitation for Client Execution",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1212",
        "technique": "Exploitation for Credential Access",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1212",
        "technique": "Exploitation for Credential Access",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1212",
        "technique": "Exploitation for Credential Access",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1211",
        "technique": "Exploitation for Defense Evasion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1211",
        "technique": "Exploitation for Defense Evasion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1211",
        "technique": "Exploitation for Defense Evasion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1068",
        "technique": "Exploitation for Privilege Escalation",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1068",
        "technique": "Exploitation for Privilege Escalation",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1068",
        "technique": "Exploitation for Privilege Escalation",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1210",
        "technique": "Exploitation of Remote Services",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1210",
        "technique": "Exploitation of Remote Services",
        "tactic": "lateral-movement"
      },
      {
        "platform": "macOS",
        "tid": "T1210",
        "technique": "Exploitation of Remote Services",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1133",
        "technique": "External Remote Services",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1133",
        "technique": "External Remote Services",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1133",
        "technique": "External Remote Services",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1133",
        "technique": "External Remote Services",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1008",
        "technique": "Fallback Channels",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1008",
        "technique": "Fallback Channels",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1008",
        "technique": "Fallback Channels",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1083",
        "technique": "File and Directory Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1083",
        "technique": "File and Directory Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1083",
        "technique": "File and Directory Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1222",
        "technique": "File and Directory Permissions Modification",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1222",
        "technique": "File and Directory Permissions Modification",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1222",
        "technique": "File and Directory Permissions Modification",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1495",
        "technique": "Firmware Corruption",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1495",
        "technique": "Firmware Corruption",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1495",
        "technique": "Firmware Corruption",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1187",
        "technique": "Forced Authentication",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1061",
        "technique": "Graphical User Interface",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1061",
        "technique": "Graphical User Interface",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1061",
        "technique": "Graphical User Interface",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1484",
        "technique": "Group Policy Modification",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1484",
        "technique": "Group Policy Modification",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1200",
        "technique": "Hardware Additions",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1200",
        "technique": "Hardware Additions",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1200",
        "technique": "Hardware Additions",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1564",
        "technique": "Hide Artifacts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1564",
        "technique": "Hide Artifacts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1564",
        "technique": "Hide Artifacts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1574",
        "technique": "Hijack Execution Flow",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1574",
        "technique": "Hijack Execution Flow",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1574",
        "technique": "Hijack Execution Flow",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1574",
        "technique": "Hijack Execution Flow",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1574",
        "technique": "Hijack Execution Flow",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1574",
        "technique": "Hijack Execution Flow",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1574",
        "technique": "Hijack Execution Flow",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1574",
        "technique": "Hijack Execution Flow",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1574",
        "technique": "Hijack Execution Flow",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1062",
        "technique": "Hypervisor",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1562",
        "technique": "Impair Defenses",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1562",
        "technique": "Impair Defenses",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1562",
        "technique": "Impair Defenses",
        "tactic": "defense-evasion"
      },
      {
        "platform": "AWS",
        "tid": "T1562",
        "technique": "Impair Defenses",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1562",
        "technique": "Impair Defenses",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1562",
        "technique": "Impair Defenses",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1525",
        "technique": "Implant Container Image",
        "tactic": "persistence"
      },
      {
        "platform": "Azure",
        "tid": "T1525",
        "technique": "Implant Container Image",
        "tactic": "persistence"
      },
      {
        "platform": "AWS",
        "tid": "T1525",
        "technique": "Implant Container Image",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1070",
        "technique": "Indicator Removal on Host",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1070",
        "technique": "Indicator Removal on Host",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1070",
        "technique": "Indicator Removal on Host",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1202",
        "technique": "Indirect Command Execution",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1105",
        "technique": "Ingress Tool Transfer",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1105",
        "technique": "Ingress Tool Transfer",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1105",
        "technique": "Ingress Tool Transfer",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1490",
        "technique": "Inhibit System Recovery",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1490",
        "technique": "Inhibit System Recovery",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1490",
        "technique": "Inhibit System Recovery",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1056",
        "technique": "Input Capture",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1056",
        "technique": "Input Capture",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1056",
        "technique": "Input Capture",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1056",
        "technique": "Input Capture",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1056",
        "technique": "Input Capture",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1056",
        "technique": "Input Capture",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1559",
        "technique": "Inter-Process Communication",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1534",
        "technique": "Internal Spearphishing",
        "tactic": "lateral-movement"
      },
      {
        "platform": "macOS",
        "tid": "T1534",
        "technique": "Internal Spearphishing",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Linux",
        "tid": "T1534",
        "technique": "Internal Spearphishing",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Office 365",
        "tid": "T1534",
        "technique": "Internal Spearphishing",
        "tactic": "lateral-movement"
      },
      {
        "platform": "SaaS",
        "tid": "T1534",
        "technique": "Internal Spearphishing",
        "tactic": "lateral-movement"
      },
      {
        "platform": "macOS",
        "tid": "T1149",
        "technique": "LC_MAIN Hijacking",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1570",
        "technique": "Lateral Tool Transfer",
        "tactic": "lateral-movement"
      },
      {
        "platform": "macOS",
        "tid": "T1570",
        "technique": "Lateral Tool Transfer",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1570",
        "technique": "Lateral Tool Transfer",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1185",
        "technique": "Man in the Browser",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1557",
        "technique": "Man-in-the-Middle",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1557",
        "technique": "Man-in-the-Middle",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1557",
        "technique": "Man-in-the-Middle",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1557",
        "technique": "Man-in-the-Middle",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1557",
        "technique": "Man-in-the-Middle",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1557",
        "technique": "Man-in-the-Middle",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1036",
        "technique": "Masquerading",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1036",
        "technique": "Masquerading",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1036",
        "technique": "Masquerading",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1556",
        "technique": "Modify Authentication Process",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1556",
        "technique": "Modify Authentication Process",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1556",
        "technique": "Modify Authentication Process",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1556",
        "technique": "Modify Authentication Process",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1556",
        "technique": "Modify Authentication Process",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1556",
        "technique": "Modify Authentication Process",
        "tactic": "defense-evasion"
      },
      {
        "platform": "AWS",
        "tid": "T1578",
        "technique": "Modify Cloud Compute Infrastructure",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1578",
        "technique": "Modify Cloud Compute Infrastructure",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1578",
        "technique": "Modify Cloud Compute Infrastructure",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1112",
        "technique": "Modify Registry",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1104",
        "technique": "Multi-Stage Channels",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1104",
        "technique": "Multi-Stage Channels",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1104",
        "technique": "Multi-Stage Channels",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1026",
        "technique": "Multiband Communication",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1026",
        "technique": "Multiband Communication",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1026",
        "technique": "Multiband Communication",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1106",
        "technique": "Native API",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1106",
        "technique": "Native API",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1106",
        "technique": "Native API",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1498",
        "technique": "Network Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1498",
        "technique": "Network Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1498",
        "technique": "Network Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "AWS",
        "tid": "T1498",
        "technique": "Network Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "GCP",
        "tid": "T1498",
        "technique": "Network Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "Azure AD",
        "tid": "T1498",
        "technique": "Network Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "SaaS",
        "tid": "T1498",
        "technique": "Network Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "Azure",
        "tid": "T1498",
        "technique": "Network Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "Office 365",
        "tid": "T1498",
        "technique": "Network Denial of Service",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1046",
        "technique": "Network Service Scanning",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1046",
        "technique": "Network Service Scanning",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1046",
        "technique": "Network Service Scanning",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1046",
        "technique": "Network Service Scanning",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1046",
        "technique": "Network Service Scanning",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1046",
        "technique": "Network Service Scanning",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1135",
        "technique": "Network Share Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1135",
        "technique": "Network Share Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1135",
        "technique": "Network Share Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1135",
        "technique": "Network Share Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1135",
        "technique": "Network Share Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1135",
        "technique": "Network Share Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1040",
        "technique": "Network Sniffing",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1040",
        "technique": "Network Sniffing",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1040",
        "technique": "Network Sniffing",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1040",
        "technique": "Network Sniffing",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1040",
        "technique": "Network Sniffing",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1040",
        "technique": "Network Sniffing",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1095",
        "technique": "Non-Application Layer Protocol",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1095",
        "technique": "Non-Application Layer Protocol",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1095",
        "technique": "Non-Application Layer Protocol",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1571",
        "technique": "Non-Standard Port",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1571",
        "technique": "Non-Standard Port",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1571",
        "technique": "Non-Standard Port",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1003",
        "technique": "OS Credential Dumping",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1003",
        "technique": "OS Credential Dumping",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1003",
        "technique": "OS Credential Dumping",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1027",
        "technique": "Obfuscated Files or Information",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1027",
        "technique": "Obfuscated Files or Information",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1027",
        "technique": "Obfuscated Files or Information",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1137",
        "technique": "Office Application Startup",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1137",
        "technique": "Office Application Startup",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1201",
        "technique": "Password Policy Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1201",
        "technique": "Password Policy Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1201",
        "technique": "Password Policy Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1120",
        "technique": "Peripheral Device Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1120",
        "technique": "Peripheral Device Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1069",
        "technique": "Permission Groups Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1069",
        "technique": "Permission Groups Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1069",
        "technique": "Permission Groups Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Office 365",
        "tid": "T1069",
        "technique": "Permission Groups Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure AD",
        "tid": "T1069",
        "technique": "Permission Groups Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1069",
        "technique": "Permission Groups Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1069",
        "technique": "Permission Groups Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1069",
        "technique": "Permission Groups Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "SaaS",
        "tid": "T1069",
        "technique": "Permission Groups Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1566",
        "technique": "Phishing",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1566",
        "technique": "Phishing",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1566",
        "technique": "Phishing",
        "tactic": "initial-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1566",
        "technique": "Phishing",
        "tactic": "initial-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1566",
        "technique": "Phishing",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1542",
        "technique": "Pre-OS Boot",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1542",
        "technique": "Pre-OS Boot",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1542",
        "technique": "Pre-OS Boot",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1542",
        "technique": "Pre-OS Boot",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1057",
        "technique": "Process Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1057",
        "technique": "Process Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1057",
        "technique": "Process Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1055",
        "technique": "Process Injection",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1055",
        "technique": "Process Injection",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1055",
        "technique": "Process Injection",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1055",
        "technique": "Process Injection",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1055",
        "technique": "Process Injection",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1055",
        "technique": "Process Injection",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1572",
        "technique": "Protocol Tunneling",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1572",
        "technique": "Protocol Tunneling",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1572",
        "technique": "Protocol Tunneling",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1090",
        "technique": "Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1090",
        "technique": "Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1090",
        "technique": "Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1012",
        "technique": "Query Registry",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "persistence"
      },
      {
        "platform": "AWS",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "defense-evasion"
      },
      {
        "platform": "AWS",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "persistence"
      },
      {
        "platform": "GCP",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "persistence"
      },
      {
        "platform": "Azure",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Office 365",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "persistence"
      },
      {
        "platform": "SaaS",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "defense-evasion"
      },
      {
        "platform": "SaaS",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "persistence"
      },
      {
        "platform": "Azure AD",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure AD",
        "tid": "T1108",
        "technique": "Redundant Access",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1219",
        "technique": "Remote Access Software",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1219",
        "technique": "Remote Access Software",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1219",
        "technique": "Remote Access Software",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1563",
        "technique": "Remote Service Session Hijacking",
        "tactic": "lateral-movement"
      },
      {
        "platform": "macOS",
        "tid": "T1563",
        "technique": "Remote Service Session Hijacking",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1563",
        "technique": "Remote Service Session Hijacking",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Linux",
        "tid": "T1021",
        "technique": "Remote Services",
        "tactic": "lateral-movement"
      },
      {
        "platform": "macOS",
        "tid": "T1021",
        "technique": "Remote Services",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1021",
        "technique": "Remote Services",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Linux",
        "tid": "T1018",
        "technique": "Remote System Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1018",
        "technique": "Remote System Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1018",
        "technique": "Remote System Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1018",
        "technique": "Remote System Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1018",
        "technique": "Remote System Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1018",
        "technique": "Remote System Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1091",
        "technique": "Replication Through Removable Media",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1091",
        "technique": "Replication Through Removable Media",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1496",
        "technique": "Resource Hijacking",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1496",
        "technique": "Resource Hijacking",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1496",
        "technique": "Resource Hijacking",
        "tactic": "impact"
      },
      {
        "platform": "AWS",
        "tid": "T1496",
        "technique": "Resource Hijacking",
        "tactic": "impact"
      },
      {
        "platform": "GCP",
        "tid": "T1496",
        "technique": "Resource Hijacking",
        "tactic": "impact"
      },
      {
        "platform": "Azure",
        "tid": "T1496",
        "technique": "Resource Hijacking",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1207",
        "technique": "Rogue Domain Controller",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1014",
        "technique": "Rootkit",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1014",
        "technique": "Rootkit",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1014",
        "technique": "Rootkit",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1053",
        "technique": "Scheduled Task/Job",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1053",
        "technique": "Scheduled Task/Job",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1053",
        "technique": "Scheduled Task/Job",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1053",
        "technique": "Scheduled Task/Job",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1053",
        "technique": "Scheduled Task/Job",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1053",
        "technique": "Scheduled Task/Job",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1053",
        "technique": "Scheduled Task/Job",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1053",
        "technique": "Scheduled Task/Job",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1053",
        "technique": "Scheduled Task/Job",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1029",
        "technique": "Scheduled Transfer",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1029",
        "technique": "Scheduled Transfer",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1029",
        "technique": "Scheduled Transfer",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1113",
        "technique": "Screen Capture",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1113",
        "technique": "Screen Capture",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1113",
        "technique": "Screen Capture",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1505",
        "technique": "Server Software Component",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1505",
        "technique": "Server Software Component",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1505",
        "technique": "Server Software Component",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1489",
        "technique": "Service Stop",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1129",
        "technique": "Shared Modules",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1051",
        "technique": "Shared Webroot",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1218",
        "technique": "Signed Binary Proxy Execution",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1216",
        "technique": "Signed Script Proxy Execution",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1072",
        "technique": "Software Deployment Tools",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1072",
        "technique": "Software Deployment Tools",
        "tactic": "lateral-movement"
      },
      {
        "platform": "macOS",
        "tid": "T1072",
        "technique": "Software Deployment Tools",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1072",
        "technique": "Software Deployment Tools",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1072",
        "technique": "Software Deployment Tools",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1072",
        "technique": "Software Deployment Tools",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Linux",
        "tid": "T1518",
        "technique": "Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1518",
        "technique": "Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1518",
        "technique": "Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1518",
        "technique": "Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1518",
        "technique": "Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1518",
        "technique": "Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Office 365",
        "tid": "T1518",
        "technique": "Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure AD",
        "tid": "T1518",
        "technique": "Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "SaaS",
        "tid": "T1518",
        "technique": "Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1153",
        "technique": "Source",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1153",
        "technique": "Source",
        "tactic": "execution"
      },
      {
        "platform": "SaaS",
        "tid": "T1528",
        "technique": "Steal Application Access Token",
        "tactic": "credential-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1528",
        "technique": "Steal Application Access Token",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure AD",
        "tid": "T1528",
        "technique": "Steal Application Access Token",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1539",
        "technique": "Steal Web Session Cookie",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1539",
        "technique": "Steal Web Session Cookie",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1539",
        "technique": "Steal Web Session Cookie",
        "tactic": "credential-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1539",
        "technique": "Steal Web Session Cookie",
        "tactic": "credential-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1539",
        "technique": "Steal Web Session Cookie",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1558",
        "technique": "Steal or Forge Kerberos Tickets",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1195",
        "technique": "Supply Chain Compromise",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1195",
        "technique": "Supply Chain Compromise",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1195",
        "technique": "Supply Chain Compromise",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1082",
        "technique": "System Information Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1082",
        "technique": "System Information Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1082",
        "technique": "System Information Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1082",
        "technique": "System Information Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1082",
        "technique": "System Information Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1082",
        "technique": "System Information Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1016",
        "technique": "System Network Configuration Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1016",
        "technique": "System Network Configuration Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1016",
        "technique": "System Network Configuration Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1049",
        "technique": "System Network Connections Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1049",
        "technique": "System Network Connections Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1049",
        "technique": "System Network Connections Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1049",
        "technique": "System Network Connections Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1049",
        "technique": "System Network Connections Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1049",
        "technique": "System Network Connections Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1033",
        "technique": "System Owner/User Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1033",
        "technique": "System Owner/User Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1033",
        "technique": "System Owner/User Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1007",
        "technique": "System Service Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1569",
        "technique": "System Services",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1569",
        "technique": "System Services",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1529",
        "technique": "System Shutdown/Reboot",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1529",
        "technique": "System Shutdown/Reboot",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1529",
        "technique": "System Shutdown/Reboot",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1124",
        "technique": "System Time Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1080",
        "technique": "Taint Shared Content",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1221",
        "technique": "Template Injection",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1537",
        "technique": "Transfer Data to Cloud Account",
        "tactic": "exfiltration"
      },
      {
        "platform": "AWS",
        "tid": "T1537",
        "technique": "Transfer Data to Cloud Account",
        "tactic": "exfiltration"
      },
      {
        "platform": "GCP",
        "tid": "T1537",
        "technique": "Transfer Data to Cloud Account",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1127",
        "technique": "Trusted Developer Utilities Proxy Execution",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1199",
        "technique": "Trusted Relationship",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1199",
        "technique": "Trusted Relationship",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1199",
        "technique": "Trusted Relationship",
        "tactic": "initial-access"
      },
      {
        "platform": "AWS",
        "tid": "T1199",
        "technique": "Trusted Relationship",
        "tactic": "initial-access"
      },
      {
        "platform": "GCP",
        "tid": "T1199",
        "technique": "Trusted Relationship",
        "tactic": "initial-access"
      },
      {
        "platform": "Azure",
        "tid": "T1199",
        "technique": "Trusted Relationship",
        "tactic": "initial-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1199",
        "technique": "Trusted Relationship",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1111",
        "technique": "Two-Factor Authentication Interception",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1111",
        "technique": "Two-Factor Authentication Interception",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1111",
        "technique": "Two-Factor Authentication Interception",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1552",
        "technique": "Unsecured Credentials",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1552",
        "technique": "Unsecured Credentials",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1552",
        "technique": "Unsecured Credentials",
        "tactic": "credential-access"
      },
      {
        "platform": "AWS",
        "tid": "T1552",
        "technique": "Unsecured Credentials",
        "tactic": "credential-access"
      },
      {
        "platform": "GCP",
        "tid": "T1552",
        "technique": "Unsecured Credentials",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure",
        "tid": "T1552",
        "technique": "Unsecured Credentials",
        "tactic": "credential-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1552",
        "technique": "Unsecured Credentials",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure AD",
        "tid": "T1552",
        "technique": "Unsecured Credentials",
        "tactic": "credential-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1552",
        "technique": "Unsecured Credentials",
        "tactic": "credential-access"
      },
      {
        "platform": "AWS",
        "tid": "T1535",
        "technique": "Unused/Unsupported Cloud Regions",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1535",
        "technique": "Unused/Unsupported Cloud Regions",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1535",
        "technique": "Unused/Unsupported Cloud Regions",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1550",
        "technique": "Use Alternate Authentication Material",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1550",
        "technique": "Use Alternate Authentication Material",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Office 365",
        "tid": "T1550",
        "technique": "Use Alternate Authentication Material",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Office 365",
        "tid": "T1550",
        "technique": "Use Alternate Authentication Material",
        "tactic": "lateral-movement"
      },
      {
        "platform": "SaaS",
        "tid": "T1550",
        "technique": "Use Alternate Authentication Material",
        "tactic": "defense-evasion"
      },
      {
        "platform": "SaaS",
        "tid": "T1550",
        "technique": "Use Alternate Authentication Material",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Linux",
        "tid": "T1204",
        "technique": "User Execution",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1204",
        "technique": "User Execution",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1204",
        "technique": "User Execution",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "AWS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "AWS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "AWS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "AWS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "GCP",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "GCP",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "GCP",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Azure",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Azure",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Azure",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "SaaS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "SaaS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "SaaS",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Office 365",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Office 365",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078",
        "technique": "Valid Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1125",
        "technique": "Video Capture",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1125",
        "technique": "Video Capture",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1497",
        "technique": "Virtualization/Sandbox Evasion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1497",
        "technique": "Virtualization/Sandbox Evasion",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1497",
        "technique": "Virtualization/Sandbox Evasion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1497",
        "technique": "Virtualization/Sandbox Evasion",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1497",
        "technique": "Virtualization/Sandbox Evasion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1497",
        "technique": "Virtualization/Sandbox Evasion",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1102",
        "technique": "Web Service",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1102",
        "technique": "Web Service",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1102",
        "technique": "Web Service",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1047",
        "technique": "Windows Management Instrumentation",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1220",
        "technique": "XSL Script Processing",
        "tactic": "defense-evasion"
      }
    ]
  },
  "breakdown_subtechniques": {
    "count": 808,
    "platforms": [
      {
        "platform": "Linux",
        "tid": "T1546.004",
        "technique": ".bash_profile and .bashrc",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1546.004",
        "technique": ".bash_profile and .bashrc",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1546.004",
        "technique": ".bash_profile and .bashrc",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1546.004",
        "technique": ".bash_profile and .bashrc",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1003.008",
        "technique": "/etc/passwd and /etc/shadow",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1546.008",
        "technique": "Accessibility Features",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.008",
        "technique": "Accessibility Features",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1098.003",
        "technique": "Add Office 365 Global Administrator Role",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1137.006",
        "technique": "Add-ins",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1137.006",
        "technique": "Add-ins",
        "tactic": "persistence"
      },
      {
        "platform": "Azure AD",
        "tid": "T1098.001",
        "technique": "Additional Azure Service Principal Credentials",
        "tactic": "persistence"
      },
      {
        "platform": "Azure",
        "tid": "T1098.001",
        "technique": "Additional Azure Service Principal Credentials",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1546.009",
        "technique": "AppCert DLLs",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.009",
        "technique": "AppCert DLLs",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1546.010",
        "technique": "AppInit DLLs",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.010",
        "technique": "AppInit DLLs",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1059.002",
        "technique": "AppleScript",
        "tactic": "execution"
      },
      {
        "platform": "Office 365",
        "tid": "T1550.001",
        "technique": "Application Access Token",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Office 365",
        "tid": "T1550.001",
        "technique": "Application Access Token",
        "tactic": "lateral-movement"
      },
      {
        "platform": "SaaS",
        "tid": "T1550.001",
        "technique": "Application Access Token",
        "tactic": "defense-evasion"
      },
      {
        "platform": "SaaS",
        "tid": "T1550.001",
        "technique": "Application Access Token",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Linux",
        "tid": "T1499.003",
        "technique": "Application Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1499.003",
        "technique": "Application Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1499.003",
        "technique": "Application Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "AWS",
        "tid": "T1499.003",
        "technique": "Application Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "GCP",
        "tid": "T1499.003",
        "technique": "Application Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Azure",
        "tid": "T1499.003",
        "technique": "Application Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Office 365",
        "tid": "T1499.003",
        "technique": "Application Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Azure AD",
        "tid": "T1499.003",
        "technique": "Application Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "SaaS",
        "tid": "T1499.003",
        "technique": "Application Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1546.011",
        "technique": "Application Shimming",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.011",
        "technique": "Application Shimming",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1499.004",
        "technique": "Application or System Exploitation",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1499.004",
        "technique": "Application or System Exploitation",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1499.004",
        "technique": "Application or System Exploitation",
        "tactic": "impact"
      },
      {
        "platform": "AWS",
        "tid": "T1499.004",
        "technique": "Application or System Exploitation",
        "tactic": "impact"
      },
      {
        "platform": "GCP",
        "tid": "T1499.004",
        "technique": "Application or System Exploitation",
        "tactic": "impact"
      },
      {
        "platform": "Azure",
        "tid": "T1499.004",
        "technique": "Application or System Exploitation",
        "tactic": "impact"
      },
      {
        "platform": "Office 365",
        "tid": "T1499.004",
        "technique": "Application or System Exploitation",
        "tactic": "impact"
      },
      {
        "platform": "Azure AD",
        "tid": "T1499.004",
        "technique": "Application or System Exploitation",
        "tactic": "impact"
      },
      {
        "platform": "SaaS",
        "tid": "T1499.004",
        "technique": "Application or System Exploitation",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1560.003",
        "technique": "Archive via Custom Method",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1560.003",
        "technique": "Archive via Custom Method",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1560.003",
        "technique": "Archive via Custom Method",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1560.002",
        "technique": "Archive via Library",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1560.002",
        "technique": "Archive via Library",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1560.002",
        "technique": "Archive via Library",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1560.001",
        "technique": "Archive via Utility",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1560.001",
        "technique": "Archive via Utility",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1560.001",
        "technique": "Archive via Utility",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1573.002",
        "technique": "Asymmetric Cryptography",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1573.002",
        "technique": "Asymmetric Cryptography",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1573.002",
        "technique": "Asymmetric Cryptography",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1055.004",
        "technique": "Asynchronous Procedure Call",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1055.004",
        "technique": "Asynchronous Procedure Call",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1053.001",
        "technique": "At (Linux)",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1053.001",
        "technique": "At (Linux)",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1053.001",
        "technique": "At (Linux)",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1053.002",
        "technique": "At (Windows)",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1053.002",
        "technique": "At (Windows)",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1053.002",
        "technique": "At (Windows)",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1547.002",
        "technique": "Authentication Package",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1547.002",
        "technique": "Authentication Package",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1552.003",
        "technique": "Bash History",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1552.003",
        "technique": "Bash History",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1102.002",
        "technique": "Bidirectional Communication",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1102.002",
        "technique": "Bidirectional Communication",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1102.002",
        "technique": "Bidirectional Communication",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1027.001",
        "technique": "Binary Padding",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1027.001",
        "technique": "Binary Padding",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1027.001",
        "technique": "Binary Padding",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1542.003",
        "technique": "Bootkit",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1542.003",
        "technique": "Bootkit",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1542.003",
        "technique": "Bootkit",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1542.003",
        "technique": "Bootkit",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1548.002",
        "technique": "Bypass User Access Control",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1548.002",
        "technique": "Bypass User Access Control",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1218.003",
        "technique": "CMSTP",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1574.012",
        "technique": "COR_PROFILER",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1574.012",
        "technique": "COR_PROFILER",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1574.012",
        "technique": "COR_PROFILER",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1003.005",
        "technique": "Cached Domain Credentials",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1546.001",
        "technique": "Change Default File Association",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.001",
        "technique": "Change Default File Association",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1070.003",
        "technique": "Clear Command History",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1070.003",
        "technique": "Clear Command History",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1070.002",
        "technique": "Clear Linux or Mac System Logs",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1070.002",
        "technique": "Clear Linux or Mac System Logs",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1070.001",
        "technique": "Clear Windows Event Logs",
        "tactic": "defense-evasion"
      },
      {
        "platform": "AWS",
        "tid": "T1136.003",
        "technique": "Cloud Account",
        "tactic": "persistence"
      },
      {
        "platform": "GCP",
        "tid": "T1136.003",
        "technique": "Cloud Account",
        "tactic": "persistence"
      },
      {
        "platform": "Azure",
        "tid": "T1136.003",
        "technique": "Cloud Account",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1136.003",
        "technique": "Cloud Account",
        "tactic": "persistence"
      },
      {
        "platform": "Azure AD",
        "tid": "T1136.003",
        "technique": "Cloud Account",
        "tactic": "persistence"
      },
      {
        "platform": "AWS",
        "tid": "T1087.004",
        "technique": "Cloud Account",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1087.004",
        "technique": "Cloud Account",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1087.004",
        "technique": "Cloud Account",
        "tactic": "discovery"
      },
      {
        "platform": "Office 365",
        "tid": "T1087.004",
        "technique": "Cloud Account",
        "tactic": "discovery"
      },
      {
        "platform": "Azure AD",
        "tid": "T1087.004",
        "technique": "Cloud Account",
        "tactic": "discovery"
      },
      {
        "platform": "SaaS",
        "tid": "T1087.004",
        "technique": "Cloud Account",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "AWS",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "AWS",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "AWS",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "GCP",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "GCP",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "GCP",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Azure",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Azure",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Azure",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "SaaS",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "SaaS",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "SaaS",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Office 365",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Office 365",
        "tid": "T1078.004",
        "technique": "Cloud Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1069.003",
        "technique": "Cloud Groups",
        "tactic": "discovery"
      },
      {
        "platform": "Azure AD",
        "tid": "T1069.003",
        "technique": "Cloud Groups",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1552.005",
        "technique": "Cloud Instance Metadata API",
        "tactic": "credential-access"
      },
      {
        "platform": "GCP",
        "tid": "T1552.005",
        "technique": "Cloud Instance Metadata API",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure",
        "tid": "T1552.005",
        "technique": "Cloud Instance Metadata API",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1553.002",
        "technique": "Code Signing",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1553.002",
        "technique": "Code Signing",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1027.004",
        "technique": "Compile After Delivery",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1027.004",
        "technique": "Compile After Delivery",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1027.004",
        "technique": "Compile After Delivery",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1218.001",
        "technique": "Compiled HTML File",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1542.002",
        "technique": "Component Firmware",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1542.002",
        "technique": "Component Firmware",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1559.001",
        "technique": "Component Object Model",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1546.015",
        "technique": "Component Object Model Hijacking",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.015",
        "technique": "Component Object Model Hijacking",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1195.003",
        "technique": "Compromise Hardware Supply Chain",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1195.003",
        "technique": "Compromise Hardware Supply Chain",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1195.003",
        "technique": "Compromise Hardware Supply Chain",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1195.001",
        "technique": "Compromise Software Dependencies and Development Tools",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1195.001",
        "technique": "Compromise Software Dependencies and Development Tools",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1195.001",
        "technique": "Compromise Software Dependencies and Development Tools",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1195.002",
        "technique": "Compromise Software Supply Chain",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1195.002",
        "technique": "Compromise Software Supply Chain",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1195.002",
        "technique": "Compromise Software Supply Chain",
        "tactic": "initial-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1213.001",
        "technique": "Confluence",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1218.002",
        "technique": "Control Panel",
        "tactic": "defense-evasion"
      },
      {
        "platform": "AWS",
        "tid": "T1578.002",
        "technique": "Create Cloud Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1578.002",
        "technique": "Create Cloud Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1578.002",
        "technique": "Create Cloud Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1134.002",
        "technique": "Create Process with Token",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1134.002",
        "technique": "Create Process with Token",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "AWS",
        "tid": "T1578.001",
        "technique": "Create Snapshot",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1578.001",
        "technique": "Create Snapshot",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1578.001",
        "technique": "Create Snapshot",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1056.004",
        "technique": "Credential API Hooking",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1056.004",
        "technique": "Credential API Hooking",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1110.004",
        "technique": "Credential Stuffing",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1110.004",
        "technique": "Credential Stuffing",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1110.004",
        "technique": "Credential Stuffing",
        "tactic": "credential-access"
      },
      {
        "platform": "AWS",
        "tid": "T1110.004",
        "technique": "Credential Stuffing",
        "tactic": "credential-access"
      },
      {
        "platform": "GCP",
        "tid": "T1110.004",
        "technique": "Credential Stuffing",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure",
        "tid": "T1110.004",
        "technique": "Credential Stuffing",
        "tactic": "credential-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1110.004",
        "technique": "Credential Stuffing",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure AD",
        "tid": "T1110.004",
        "technique": "Credential Stuffing",
        "tactic": "credential-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1110.004",
        "technique": "Credential Stuffing",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1552.001",
        "technique": "Credentials In Files",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1552.001",
        "technique": "Credentials In Files",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1552.001",
        "technique": "Credentials In Files",
        "tactic": "credential-access"
      },
      {
        "platform": "AWS",
        "tid": "T1552.001",
        "technique": "Credentials In Files",
        "tactic": "credential-access"
      },
      {
        "platform": "GCP",
        "tid": "T1552.001",
        "technique": "Credentials In Files",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure",
        "tid": "T1552.001",
        "technique": "Credentials In Files",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1555.003",
        "technique": "Credentials from Web Browsers",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1555.003",
        "technique": "Credentials from Web Browsers",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1555.003",
        "technique": "Credentials from Web Browsers",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1552.002",
        "technique": "Credentials in Registry",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1053.003",
        "technique": "Cron",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1053.003",
        "technique": "Cron",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1053.003",
        "technique": "Cron",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1053.003",
        "technique": "Cron",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1053.003",
        "technique": "Cron",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1053.003",
        "technique": "Cron",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1003.006",
        "technique": "DCSync",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1574.001",
        "technique": "DLL Search Order Hijacking",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1574.001",
        "technique": "DLL Search Order Hijacking",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1574.001",
        "technique": "DLL Search Order Hijacking",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1574.002",
        "technique": "DLL Side-Loading",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1574.002",
        "technique": "DLL Side-Loading",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1574.002",
        "technique": "DLL Side-Loading",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1071.004",
        "technique": "DNS",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1071.004",
        "technique": "DNS",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1071.004",
        "technique": "DNS",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1568.003",
        "technique": "DNS Calculation",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1568.003",
        "technique": "DNS Calculation",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1568.003",
        "technique": "DNS Calculation",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1102.001",
        "technique": "Dead Drop Resolver",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1102.001",
        "technique": "Dead Drop Resolver",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1102.001",
        "technique": "Dead Drop Resolver",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "AWS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "AWS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "AWS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "AWS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "GCP",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "GCP",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "GCP",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Azure",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Azure",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Azure",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Office 365",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Office 365",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Azure AD",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "SaaS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "SaaS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "SaaS",
        "tid": "T1078.001",
        "technique": "Default Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "AWS",
        "tid": "T1578.003",
        "technique": "Delete Cloud Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1578.003",
        "technique": "Delete Cloud Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1578.003",
        "technique": "Delete Cloud Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1498.001",
        "technique": "Direct Network Flood",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1498.001",
        "technique": "Direct Network Flood",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1498.001",
        "technique": "Direct Network Flood",
        "tactic": "impact"
      },
      {
        "platform": "AWS",
        "tid": "T1498.001",
        "technique": "Direct Network Flood",
        "tactic": "impact"
      },
      {
        "platform": "GCP",
        "tid": "T1498.001",
        "technique": "Direct Network Flood",
        "tactic": "impact"
      },
      {
        "platform": "Azure AD",
        "tid": "T1498.001",
        "technique": "Direct Network Flood",
        "tactic": "impact"
      },
      {
        "platform": "SaaS",
        "tid": "T1498.001",
        "technique": "Direct Network Flood",
        "tactic": "impact"
      },
      {
        "platform": "Azure",
        "tid": "T1498.001",
        "technique": "Direct Network Flood",
        "tactic": "impact"
      },
      {
        "platform": "Office 365",
        "tid": "T1498.001",
        "technique": "Direct Network Flood",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1562.002",
        "technique": "Disable Windows Event Logging",
        "tactic": "defense-evasion"
      },
      {
        "platform": "AWS",
        "tid": "T1562.007",
        "technique": "Disable or Modify Cloud Firewall",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1562.007",
        "technique": "Disable or Modify Cloud Firewall",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1562.007",
        "technique": "Disable or Modify Cloud Firewall",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1562.004",
        "technique": "Disable or Modify System Firewall",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1562.004",
        "technique": "Disable or Modify System Firewall",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1562.004",
        "technique": "Disable or Modify System Firewall",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1562.001",
        "technique": "Disable or Modify Tools",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1562.001",
        "technique": "Disable or Modify Tools",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1562.001",
        "technique": "Disable or Modify Tools",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1561.001",
        "technique": "Disk Content Wipe",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1561.001",
        "technique": "Disk Content Wipe",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1561.001",
        "technique": "Disk Content Wipe",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1561.002",
        "technique": "Disk Structure Wipe",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1561.002",
        "technique": "Disk Structure Wipe",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1561.002",
        "technique": "Disk Structure Wipe",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1021.003",
        "technique": "Distributed Component Object Model",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1136.002",
        "technique": "Domain Account",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1136.002",
        "technique": "Domain Account",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1136.002",
        "technique": "Domain Account",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1087.002",
        "technique": "Domain Account",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1087.002",
        "technique": "Domain Account",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1087.002",
        "technique": "Domain Account",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1078.002",
        "technique": "Domain Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1556.001",
        "technique": "Domain Controller Authentication",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1556.001",
        "technique": "Domain Controller Authentication",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1090.004",
        "technique": "Domain Fronting",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1090.004",
        "technique": "Domain Fronting",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1090.004",
        "technique": "Domain Fronting",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1568.002",
        "technique": "Domain Generation Algorithms",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1568.002",
        "technique": "Domain Generation Algorithms",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1568.002",
        "technique": "Domain Generation Algorithms",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1069.002",
        "technique": "Domain Groups",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1069.002",
        "technique": "Domain Groups",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1069.002",
        "technique": "Domain Groups",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1574.004",
        "technique": "Dylib Hijacking",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1574.004",
        "technique": "Dylib Hijacking",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1574.004",
        "technique": "Dylib Hijacking",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1559.002",
        "technique": "Dynamic Data Exchange",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1055.001",
        "technique": "Dynamic-link Library Injection",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1055.001",
        "technique": "Dynamic-link Library Injection",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1548.004",
        "technique": "Elevated Execution with Prompt",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1548.004",
        "technique": "Elevated Execution with Prompt",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1087.003",
        "technique": "Email Account",
        "tactic": "discovery"
      },
      {
        "platform": "Office 365",
        "tid": "T1087.003",
        "technique": "Email Account",
        "tactic": "discovery"
      },
      {
        "platform": "Office 365",
        "tid": "T1114.003",
        "technique": "Email Forwarding Rule",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1114.003",
        "technique": "Email Forwarding Rule",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1546.014",
        "technique": "Emond",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1546.014",
        "technique": "Emond",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1480.001",
        "technique": "Environmental Keying",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1480.001",
        "technique": "Environmental Keying",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1480.001",
        "technique": "Environmental Keying",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1098.002",
        "technique": "Exchange Email Delegate Permissions",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1098.002",
        "technique": "Exchange Email Delegate Permissions",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1574.005",
        "technique": "Executable Installer File Permissions Weakness",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1574.005",
        "technique": "Executable Installer File Permissions Weakness",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1574.005",
        "technique": "Executable Installer File Permissions Weakness",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1048.002",
        "technique": "Exfiltration Over Asymmetric Encrypted Non-C2 Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1048.002",
        "technique": "Exfiltration Over Asymmetric Encrypted Non-C2 Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1048.002",
        "technique": "Exfiltration Over Asymmetric Encrypted Non-C2 Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1011.001",
        "technique": "Exfiltration Over Bluetooth",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1011.001",
        "technique": "Exfiltration Over Bluetooth",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1011.001",
        "technique": "Exfiltration Over Bluetooth",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1048.001",
        "technique": "Exfiltration Over Symmetric Encrypted Non-C2 Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1048.001",
        "technique": "Exfiltration Over Symmetric Encrypted Non-C2 Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1048.001",
        "technique": "Exfiltration Over Symmetric Encrypted Non-C2 Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1048.003",
        "technique": "Exfiltration Over Unencrypted/Obfuscated Non-C2 Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1048.003",
        "technique": "Exfiltration Over Unencrypted/Obfuscated Non-C2 Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1048.003",
        "technique": "Exfiltration Over Unencrypted/Obfuscated Non-C2 Protocol",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1052.001",
        "technique": "Exfiltration over USB",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1052.001",
        "technique": "Exfiltration over USB",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1052.001",
        "technique": "Exfiltration over USB",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1567.002",
        "technique": "Exfiltration to Cloud Storage",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1567.002",
        "technique": "Exfiltration to Cloud Storage",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1567.002",
        "technique": "Exfiltration to Cloud Storage",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1567.001",
        "technique": "Exfiltration to Code Repository",
        "tactic": "exfiltration"
      },
      {
        "platform": "macOS",
        "tid": "T1567.001",
        "technique": "Exfiltration to Code Repository",
        "tactic": "exfiltration"
      },
      {
        "platform": "Windows",
        "tid": "T1567.001",
        "technique": "Exfiltration to Code Repository",
        "tactic": "exfiltration"
      },
      {
        "platform": "Linux",
        "tid": "T1491.002",
        "technique": "External Defacement",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1491.002",
        "technique": "External Defacement",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1491.002",
        "technique": "External Defacement",
        "tactic": "impact"
      },
      {
        "platform": "AWS",
        "tid": "T1491.002",
        "technique": "External Defacement",
        "tactic": "impact"
      },
      {
        "platform": "GCP",
        "tid": "T1491.002",
        "technique": "External Defacement",
        "tactic": "impact"
      },
      {
        "platform": "Azure",
        "tid": "T1491.002",
        "technique": "External Defacement",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1090.002",
        "technique": "External Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1090.002",
        "technique": "External Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1090.002",
        "technique": "External Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1055.011",
        "technique": "Extra Window Memory Injection",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1055.011",
        "technique": "Extra Window Memory Injection",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1568.001",
        "technique": "Fast Flux DNS",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1568.001",
        "technique": "Fast Flux DNS",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1568.001",
        "technique": "Fast Flux DNS",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1070.004",
        "technique": "File Deletion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1070.004",
        "technique": "File Deletion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1070.004",
        "technique": "File Deletion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1071.002",
        "technique": "File Transfer Protocols",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1071.002",
        "technique": "File Transfer Protocols",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1071.002",
        "technique": "File Transfer Protocols",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1056.002",
        "technique": "GUI Input Capture",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1056.002",
        "technique": "GUI Input Capture",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1056.002",
        "technique": "GUI Input Capture",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1056.002",
        "technique": "GUI Input Capture",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1553.001",
        "technique": "Gatekeeper Bypass",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1558.001",
        "technique": "Golden Ticket",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1552.006",
        "technique": "Group Policy Preferences",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1562.003",
        "technique": "HISTCONTROL",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1562.003",
        "technique": "HISTCONTROL",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1564.005",
        "technique": "Hidden File System",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1564.005",
        "technique": "Hidden File System",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1564.005",
        "technique": "Hidden File System",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1564.001",
        "technique": "Hidden Files and Directories",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1564.001",
        "technique": "Hidden Files and Directories",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1564.001",
        "technique": "Hidden Files and Directories",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1564.002",
        "technique": "Hidden Users",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1564.003",
        "technique": "Hidden Window",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1564.003",
        "technique": "Hidden Window",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1546.012",
        "technique": "Image File Execution Options Injection",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.012",
        "technique": "Image File Execution Options Injection",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1562.006",
        "technique": "Indicator Blocking",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1027.005",
        "technique": "Indicator Removal from Tools",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1027.005",
        "technique": "Indicator Removal from Tools",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1027.005",
        "technique": "Indicator Removal from Tools",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1553.004",
        "technique": "Install Root Certificate",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1553.004",
        "technique": "Install Root Certificate",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1553.004",
        "technique": "Install Root Certificate",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1218.004",
        "technique": "InstallUtil",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1491.001",
        "technique": "Internal Defacement",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1491.001",
        "technique": "Internal Defacement",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1491.001",
        "technique": "Internal Defacement",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1090.001",
        "technique": "Internal Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1090.001",
        "technique": "Internal Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1090.001",
        "technique": "Internal Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1036.001",
        "technique": "Invalid Code Signature",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1036.001",
        "technique": "Invalid Code Signature",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1059.007",
        "technique": "JavaScript/JScript",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1059.007",
        "technique": "JavaScript/JScript",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1059.007",
        "technique": "JavaScript/JScript",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1001.001",
        "technique": "Junk Data",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1001.001",
        "technique": "Junk Data",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1001.001",
        "technique": "Junk Data",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1558.003",
        "technique": "Kerberoasting",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1547.006",
        "technique": "Kernel Modules and Extensions",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1547.006",
        "technique": "Kernel Modules and Extensions",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1547.006",
        "technique": "Kernel Modules and Extensions",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1547.006",
        "technique": "Kernel Modules and Extensions",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1555.001",
        "technique": "Keychain",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1056.001",
        "technique": "Keylogging",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1056.001",
        "technique": "Keylogging",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1056.001",
        "technique": "Keylogging",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1056.001",
        "technique": "Keylogging",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1056.001",
        "technique": "Keylogging",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1056.001",
        "technique": "Keylogging",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1546.006",
        "technique": "LC_LOAD_DYLIB Addition",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1546.006",
        "technique": "LC_LOAD_DYLIB Addition",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1574.006",
        "technique": "LD_PRELOAD",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1574.006",
        "technique": "LD_PRELOAD",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1574.006",
        "technique": "LD_PRELOAD",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1557.001",
        "technique": "LLMNR/NBT-NS Poisoning and SMB Relay",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1557.001",
        "technique": "LLMNR/NBT-NS Poisoning and SMB Relay",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1003.004",
        "technique": "LSA Secrets",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1547.008",
        "technique": "LSASS Driver",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1547.008",
        "technique": "LSASS Driver",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1003.001",
        "technique": "LSASS Memory",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1543.001",
        "technique": "Launch Agent",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1543.001",
        "technique": "Launch Agent",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1543.004",
        "technique": "Launch Daemon",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1543.004",
        "technique": "Launch Daemon",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1569.001",
        "technique": "Launchctl",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1053.004",
        "technique": "Launchd",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1053.004",
        "technique": "Launchd",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1053.004",
        "technique": "Launchd",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1222.002",
        "technique": "Linux and Mac File and Directory Permissions Modification",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1222.002",
        "technique": "Linux and Mac File and Directory Permissions Modification",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1136.001",
        "technique": "Local Account",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1136.001",
        "technique": "Local Account",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1136.001",
        "technique": "Local Account",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1087.001",
        "technique": "Local Account",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1087.001",
        "technique": "Local Account",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1087.001",
        "technique": "Local Account",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1078.003",
        "technique": "Local Accounts",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1074.001",
        "technique": "Local Data Staging",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1074.001",
        "technique": "Local Data Staging",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1074.001",
        "technique": "Local Data Staging",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1114.001",
        "technique": "Local Email Collection",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1069.001",
        "technique": "Local Groups",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1069.001",
        "technique": "Local Groups",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1069.001",
        "technique": "Local Groups",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1037.002",
        "technique": "Logon Script (Mac)",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1037.002",
        "technique": "Logon Script (Mac)",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1037.001",
        "technique": "Logon Script (Windows)",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1037.001",
        "technique": "Logon Script (Windows)",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1127.001",
        "technique": "MSBuild",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1071.003",
        "technique": "Mail Protocols",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1071.003",
        "technique": "Mail Protocols",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1071.003",
        "technique": "Mail Protocols",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1134.003",
        "technique": "Make and Impersonate Token",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1134.003",
        "technique": "Make and Impersonate Token",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1204.002",
        "technique": "Malicious File",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1204.002",
        "technique": "Malicious File",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1204.002",
        "technique": "Malicious File",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1204.001",
        "technique": "Malicious Link",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1204.001",
        "technique": "Malicious Link",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1204.001",
        "technique": "Malicious Link",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1036.004",
        "technique": "Masquerade Task or Service",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1036.004",
        "technique": "Masquerade Task or Service",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1036.005",
        "technique": "Match Legitimate Name or Location",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1036.005",
        "technique": "Match Legitimate Name or Location",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1036.005",
        "technique": "Match Legitimate Name or Location",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1218.005",
        "technique": "Mshta",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1218.007",
        "technique": "Msiexec",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1090.003",
        "technique": "Multi-hop Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1090.003",
        "technique": "Multi-hop Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1090.003",
        "technique": "Multi-hop Proxy",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1003.003",
        "technique": "NTDS",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1564.004",
        "technique": "NTFS File Attributes",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1546.007",
        "technique": "Netsh Helper DLL",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.007",
        "technique": "Netsh Helper DLL",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1037.003",
        "technique": "Network Logon Script",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1037.003",
        "technique": "Network Logon Script",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1070.005",
        "technique": "Network Share Connection Removal",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1132.002",
        "technique": "Non-Standard Encoding",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1132.002",
        "technique": "Non-Standard Encoding",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1132.002",
        "technique": "Non-Standard Encoding",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1499.001",
        "technique": "OS Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1499.001",
        "technique": "OS Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1499.001",
        "technique": "OS Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1218.008",
        "technique": "Odbcconf",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1137.001",
        "technique": "Office Template Macros",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1137.001",
        "technique": "Office Template Macros",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1137.002",
        "technique": "Office Test",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1137.002",
        "technique": "Office Test",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1102.003",
        "technique": "One-Way Communication",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1102.003",
        "technique": "One-Way Communication",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1102.003",
        "technique": "One-Way Communication",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1137.003",
        "technique": "Outlook Forms",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1137.003",
        "technique": "Outlook Forms",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1137.004",
        "technique": "Outlook Home Page",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1137.004",
        "technique": "Outlook Home Page",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1137.005",
        "technique": "Outlook Rules",
        "tactic": "persistence"
      },
      {
        "platform": "Office 365",
        "tid": "T1137.005",
        "technique": "Outlook Rules",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1134.004",
        "technique": "Parent PID Spoofing",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1134.004",
        "technique": "Parent PID Spoofing",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1550.002",
        "technique": "Pass the Hash",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1550.002",
        "technique": "Pass the Hash",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1550.003",
        "technique": "Pass the Ticket",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1550.003",
        "technique": "Pass the Ticket",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Linux",
        "tid": "T1110.002",
        "technique": "Password Cracking",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1110.002",
        "technique": "Password Cracking",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1110.002",
        "technique": "Password Cracking",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure",
        "tid": "T1110.002",
        "technique": "Password Cracking",
        "tactic": "credential-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1110.002",
        "technique": "Password Cracking",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure AD",
        "tid": "T1110.002",
        "technique": "Password Cracking",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1556.002",
        "technique": "Password Filter DLL",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1556.002",
        "technique": "Password Filter DLL",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1110.001",
        "technique": "Password Guessing",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1110.001",
        "technique": "Password Guessing",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1110.001",
        "technique": "Password Guessing",
        "tactic": "credential-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1110.001",
        "technique": "Password Guessing",
        "tactic": "credential-access"
      },
      {
        "platform": "GCP",
        "tid": "T1110.001",
        "technique": "Password Guessing",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure AD",
        "tid": "T1110.001",
        "technique": "Password Guessing",
        "tactic": "credential-access"
      },
      {
        "platform": "AWS",
        "tid": "T1110.001",
        "technique": "Password Guessing",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure",
        "tid": "T1110.001",
        "technique": "Password Guessing",
        "tactic": "credential-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1110.001",
        "technique": "Password Guessing",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1110.003",
        "technique": "Password Spraying",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1110.003",
        "technique": "Password Spraying",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1110.003",
        "technique": "Password Spraying",
        "tactic": "credential-access"
      },
      {
        "platform": "AWS",
        "tid": "T1110.003",
        "technique": "Password Spraying",
        "tactic": "credential-access"
      },
      {
        "platform": "GCP",
        "tid": "T1110.003",
        "technique": "Password Spraying",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure",
        "tid": "T1110.003",
        "technique": "Password Spraying",
        "tactic": "credential-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1110.003",
        "technique": "Password Spraying",
        "tactic": "credential-access"
      },
      {
        "platform": "Azure AD",
        "tid": "T1110.003",
        "technique": "Password Spraying",
        "tactic": "credential-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1110.003",
        "technique": "Password Spraying",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1574.007",
        "technique": "Path Interception by PATH Environment Variable",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1574.007",
        "technique": "Path Interception by PATH Environment Variable",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1574.007",
        "technique": "Path Interception by PATH Environment Variable",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1574.008",
        "technique": "Path Interception by Search Order Hijacking",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1574.008",
        "technique": "Path Interception by Search Order Hijacking",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1574.008",
        "technique": "Path Interception by Search Order Hijacking",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1574.009",
        "technique": "Path Interception by Unquoted Path",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1574.009",
        "technique": "Path Interception by Unquoted Path",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1574.009",
        "technique": "Path Interception by Unquoted Path",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1547.011",
        "technique": "Plist Modification",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1547.011",
        "technique": "Plist Modification",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1556.003",
        "technique": "Pluggable Authentication Modules",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1556.003",
        "technique": "Pluggable Authentication Modules",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1556.003",
        "technique": "Pluggable Authentication Modules",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1556.003",
        "technique": "Pluggable Authentication Modules",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1556.003",
        "technique": "Pluggable Authentication Modules",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1556.003",
        "technique": "Pluggable Authentication Modules",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1205.001",
        "technique": "Port Knocking",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1205.001",
        "technique": "Port Knocking",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1205.001",
        "technique": "Port Knocking",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1205.001",
        "technique": "Port Knocking",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1205.001",
        "technique": "Port Knocking",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1205.001",
        "technique": "Port Knocking",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1205.001",
        "technique": "Port Knocking",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1205.001",
        "technique": "Port Knocking",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1205.001",
        "technique": "Port Knocking",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1547.010",
        "technique": "Port Monitors",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1547.010",
        "technique": "Port Monitors",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1055.002",
        "technique": "Portable Executable Injection",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1055.002",
        "technique": "Portable Executable Injection",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1059.001",
        "technique": "PowerShell",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1546.013",
        "technique": "PowerShell Profile",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.013",
        "technique": "PowerShell Profile",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1552.004",
        "technique": "Private Keys",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1552.004",
        "technique": "Private Keys",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1552.004",
        "technique": "Private Keys",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1003.007",
        "technique": "Proc Filesystem",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1055.009",
        "technique": "Proc Memory",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1055.009",
        "technique": "Proc Memory",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1055.013",
        "technique": "Process Doppelgänging",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1055.013",
        "technique": "Process Doppelgänging",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1055.012",
        "technique": "Process Hollowing",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1055.012",
        "technique": "Process Hollowing",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1001.003",
        "technique": "Protocol Impersonation",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1001.003",
        "technique": "Protocol Impersonation",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1001.003",
        "technique": "Protocol Impersonation",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1055.008",
        "technique": "Ptrace System Calls",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1055.008",
        "technique": "Ptrace System Calls",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1216.001",
        "technique": "PubPrn",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1059.006",
        "technique": "Python",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1059.006",
        "technique": "Python",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1059.006",
        "technique": "Python",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1563.002",
        "technique": "RDP Hijacking",
        "tactic": "lateral-movement"
      },
      {
        "platform": "macOS",
        "tid": "T1037.004",
        "technique": "Rc.common",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1037.004",
        "technique": "Rc.common",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1547.007",
        "technique": "Re-opened Applications",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1547.007",
        "technique": "Re-opened Applications",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1498.002",
        "technique": "Reflection Amplification",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1498.002",
        "technique": "Reflection Amplification",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1498.002",
        "technique": "Reflection Amplification",
        "tactic": "impact"
      },
      {
        "platform": "AWS",
        "tid": "T1498.002",
        "technique": "Reflection Amplification",
        "tactic": "impact"
      },
      {
        "platform": "Office 365",
        "tid": "T1498.002",
        "technique": "Reflection Amplification",
        "tactic": "impact"
      },
      {
        "platform": "Azure AD",
        "tid": "T1498.002",
        "technique": "Reflection Amplification",
        "tactic": "impact"
      },
      {
        "platform": "GCP",
        "tid": "T1498.002",
        "technique": "Reflection Amplification",
        "tactic": "impact"
      },
      {
        "platform": "Azure",
        "tid": "T1498.002",
        "technique": "Reflection Amplification",
        "tactic": "impact"
      },
      {
        "platform": "SaaS",
        "tid": "T1498.002",
        "technique": "Reflection Amplification",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1547.001",
        "technique": "Registry Run Keys / Startup Folder",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1547.001",
        "technique": "Registry Run Keys / Startup Folder",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1218.009",
        "technique": "Regsvcs/Regasm",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1218.010",
        "technique": "Regsvr32",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1074.002",
        "technique": "Remote Data Staging",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1074.002",
        "technique": "Remote Data Staging",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1074.002",
        "technique": "Remote Data Staging",
        "tactic": "collection"
      },
      {
        "platform": "AWS",
        "tid": "T1074.002",
        "technique": "Remote Data Staging",
        "tactic": "collection"
      },
      {
        "platform": "GCP",
        "tid": "T1074.002",
        "technique": "Remote Data Staging",
        "tactic": "collection"
      },
      {
        "platform": "Azure",
        "tid": "T1074.002",
        "technique": "Remote Data Staging",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1021.001",
        "technique": "Remote Desktop Protocol",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Office 365",
        "tid": "T1114.002",
        "technique": "Remote Email Collection",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1114.002",
        "technique": "Remote Email Collection",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1036.003",
        "technique": "Rename System Utilities",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1036.003",
        "technique": "Rename System Utilities",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1036.003",
        "technique": "Rename System Utilities",
        "tactic": "defense-evasion"
      },
      {
        "platform": "AWS",
        "tid": "T1578.004",
        "technique": "Revert Cloud Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "GCP",
        "tid": "T1578.004",
        "technique": "Revert Cloud Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Azure",
        "tid": "T1578.004",
        "technique": "Revert Cloud Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1036.002",
        "technique": "Right-to-Left Override",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1036.002",
        "technique": "Right-to-Left Override",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1036.002",
        "technique": "Right-to-Left Override",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1564.006",
        "technique": "Run Virtual Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1564.006",
        "technique": "Run Virtual Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1564.006",
        "technique": "Run Virtual Instance",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1218.011",
        "technique": "Rundll32",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1565.003",
        "technique": "Runtime Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1565.003",
        "technique": "Runtime Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1565.003",
        "technique": "Runtime Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1134.005",
        "technique": "SID-History Injection",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1134.005",
        "technique": "SID-History Injection",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1553.003",
        "technique": "SIP and Trust Provider Hijacking",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1021.002",
        "technique": "SMB/Windows Admin Shares",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1505.001",
        "technique": "SQL Stored Procedures",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1505.001",
        "technique": "SQL Stored Procedures",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1021.004",
        "technique": "SSH",
        "tactic": "lateral-movement"
      },
      {
        "platform": "macOS",
        "tid": "T1021.004",
        "technique": "SSH",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Linux",
        "tid": "T1098.004",
        "technique": "SSH Authorized Keys",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1098.004",
        "technique": "SSH Authorized Keys",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1563.001",
        "technique": "SSH Hijacking",
        "tactic": "lateral-movement"
      },
      {
        "platform": "macOS",
        "tid": "T1563.001",
        "technique": "SSH Hijacking",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1053.005",
        "technique": "Scheduled Task",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1053.005",
        "technique": "Scheduled Task",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1053.005",
        "technique": "Scheduled Task",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.002",
        "technique": "Screensaver",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.002",
        "technique": "Screensaver",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1003.002",
        "technique": "Security Account Manager",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1518.001",
        "technique": "Security Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1518.001",
        "technique": "Security Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1518.001",
        "technique": "Security Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "AWS",
        "tid": "T1518.001",
        "technique": "Security Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "GCP",
        "tid": "T1518.001",
        "technique": "Security Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure",
        "tid": "T1518.001",
        "technique": "Security Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Office 365",
        "tid": "T1518.001",
        "technique": "Security Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Azure AD",
        "tid": "T1518.001",
        "technique": "Security Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "SaaS",
        "tid": "T1518.001",
        "technique": "Security Software Discovery",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1547.005",
        "technique": "Security Support Provider",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1547.005",
        "technique": "Security Support Provider",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1555.002",
        "technique": "Securityd Memory",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1555.002",
        "technique": "Securityd Memory",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1569.002",
        "technique": "Service Execution",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1499.002",
        "technique": "Service Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1499.002",
        "technique": "Service Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1499.002",
        "technique": "Service Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "AWS",
        "tid": "T1499.002",
        "technique": "Service Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "GCP",
        "tid": "T1499.002",
        "technique": "Service Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Azure",
        "tid": "T1499.002",
        "technique": "Service Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Office 365",
        "tid": "T1499.002",
        "technique": "Service Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Azure AD",
        "tid": "T1499.002",
        "technique": "Service Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "SaaS",
        "tid": "T1499.002",
        "technique": "Service Exhaustion Flood",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1574.010",
        "technique": "Services File Permissions Weakness",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1574.010",
        "technique": "Services File Permissions Weakness",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1574.010",
        "technique": "Services File Permissions Weakness",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1574.011",
        "technique": "Services Registry Permissions Weakness",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1574.011",
        "technique": "Services Registry Permissions Weakness",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1574.011",
        "technique": "Services Registry Permissions Weakness",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1548.001",
        "technique": "Setuid and Setgid",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1548.001",
        "technique": "Setuid and Setgid",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1548.001",
        "technique": "Setuid and Setgid",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1548.001",
        "technique": "Setuid and Setgid",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1213.002",
        "technique": "Sharepoint",
        "tactic": "collection"
      },
      {
        "platform": "Office 365",
        "tid": "T1213.002",
        "technique": "Sharepoint",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1547.009",
        "technique": "Shortcut Modification",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1547.009",
        "technique": "Shortcut Modification",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1558.002",
        "technique": "Silver Ticket",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1027.002",
        "technique": "Software Packing",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1027.002",
        "technique": "Software Packing",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1036.006",
        "technique": "Space after Filename",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1036.006",
        "technique": "Space after Filename",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1566.001",
        "technique": "Spearphishing Attachment",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1566.001",
        "technique": "Spearphishing Attachment",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1566.001",
        "technique": "Spearphishing Attachment",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1566.002",
        "technique": "Spearphishing Link",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1566.002",
        "technique": "Spearphishing Link",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1566.002",
        "technique": "Spearphishing Link",
        "tactic": "initial-access"
      },
      {
        "platform": "Office 365",
        "tid": "T1566.002",
        "technique": "Spearphishing Link",
        "tactic": "initial-access"
      },
      {
        "platform": "SaaS",
        "tid": "T1566.002",
        "technique": "Spearphishing Link",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1566.003",
        "technique": "Spearphishing via Service",
        "tactic": "initial-access"
      },
      {
        "platform": "macOS",
        "tid": "T1566.003",
        "technique": "Spearphishing via Service",
        "tactic": "initial-access"
      },
      {
        "platform": "Windows",
        "tid": "T1566.003",
        "technique": "Spearphishing via Service",
        "tactic": "initial-access"
      },
      {
        "platform": "Linux",
        "tid": "T1132.001",
        "technique": "Standard Encoding",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1132.001",
        "technique": "Standard Encoding",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1132.001",
        "technique": "Standard Encoding",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1037.005",
        "technique": "Startup Items",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1037.005",
        "technique": "Startup Items",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1027.003",
        "technique": "Steganography",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1027.003",
        "technique": "Steganography",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1027.003",
        "technique": "Steganography",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1001.002",
        "technique": "Steganography",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1001.002",
        "technique": "Steganography",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1001.002",
        "technique": "Steganography",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1565.001",
        "technique": "Stored Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1565.001",
        "technique": "Stored Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1565.001",
        "technique": "Stored Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1548.003",
        "technique": "Sudo and Sudo Caching",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1548.003",
        "technique": "Sudo and Sudo Caching",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1548.003",
        "technique": "Sudo and Sudo Caching",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1548.003",
        "technique": "Sudo and Sudo Caching",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1573.001",
        "technique": "Symmetric Cryptography",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1573.001",
        "technique": "Symmetric Cryptography",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1573.001",
        "technique": "Symmetric Cryptography",
        "tactic": "command-and-control"
      },
      {
        "platform": "Linux",
        "tid": "T1497.001",
        "technique": "System Checks",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1497.001",
        "technique": "System Checks",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1497.001",
        "technique": "System Checks",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1497.001",
        "technique": "System Checks",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1497.001",
        "technique": "System Checks",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1497.001",
        "technique": "System Checks",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1542.001",
        "technique": "System Firmware",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1542.001",
        "technique": "System Firmware",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1543.002",
        "technique": "Systemd Service",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1543.002",
        "technique": "Systemd Service",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1055.003",
        "technique": "Thread Execution Hijacking",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1055.003",
        "technique": "Thread Execution Hijacking",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1055.005",
        "technique": "Thread Local Storage",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1055.005",
        "technique": "Thread Local Storage",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1497.003",
        "technique": "Time Based Evasion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1497.003",
        "technique": "Time Based Evasion",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1497.003",
        "technique": "Time Based Evasion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1497.003",
        "technique": "Time Based Evasion",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1497.003",
        "technique": "Time Based Evasion",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1497.003",
        "technique": "Time Based Evasion",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1547.003",
        "technique": "Time Providers",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1547.003",
        "technique": "Time Providers",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1070.006",
        "technique": "Timestomp",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1070.006",
        "technique": "Timestomp",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1070.006",
        "technique": "Timestomp",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1134.001",
        "technique": "Token Impersonation/Theft",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1134.001",
        "technique": "Token Impersonation/Theft",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1565.002",
        "technique": "Transmitted Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "macOS",
        "tid": "T1565.002",
        "technique": "Transmitted Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "Windows",
        "tid": "T1565.002",
        "technique": "Transmitted Data Manipulation",
        "tactic": "impact"
      },
      {
        "platform": "Linux",
        "tid": "T1505.002",
        "technique": "Transport Agent",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1505.002",
        "technique": "Transport Agent",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1546.005",
        "technique": "Trap",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "macOS",
        "tid": "T1546.005",
        "technique": "Trap",
        "tactic": "persistence"
      },
      {
        "platform": "Linux",
        "tid": "T1546.005",
        "technique": "Trap",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1546.005",
        "technique": "Trap",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1059.004",
        "technique": "Unix Shell",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1059.004",
        "technique": "Unix Shell",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1497.002",
        "technique": "User Activity Based Checks",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1497.002",
        "technique": "User Activity Based Checks",
        "tactic": "discovery"
      },
      {
        "platform": "macOS",
        "tid": "T1497.002",
        "technique": "User Activity Based Checks",
        "tactic": "defense-evasion"
      },
      {
        "platform": "macOS",
        "tid": "T1497.002",
        "technique": "User Activity Based Checks",
        "tactic": "discovery"
      },
      {
        "platform": "Windows",
        "tid": "T1497.002",
        "technique": "User Activity Based Checks",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1497.002",
        "technique": "User Activity Based Checks",
        "tactic": "discovery"
      },
      {
        "platform": "Linux",
        "tid": "T1055.014",
        "technique": "VDSO Hijacking",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Linux",
        "tid": "T1055.014",
        "technique": "VDSO Hijacking",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Linux",
        "tid": "T1021.005",
        "technique": "VNC",
        "tactic": "lateral-movement"
      },
      {
        "platform": "macOS",
        "tid": "T1021.005",
        "technique": "VNC",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1021.005",
        "technique": "VNC",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1059.005",
        "technique": "Visual Basic",
        "tactic": "execution"
      },
      {
        "platform": "macOS",
        "tid": "T1059.005",
        "technique": "Visual Basic",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1059.005",
        "technique": "Visual Basic",
        "tactic": "execution"
      },
      {
        "platform": "Linux",
        "tid": "T1056.003",
        "technique": "Web Portal Capture",
        "tactic": "collection"
      },
      {
        "platform": "Linux",
        "tid": "T1056.003",
        "technique": "Web Portal Capture",
        "tactic": "credential-access"
      },
      {
        "platform": "macOS",
        "tid": "T1056.003",
        "technique": "Web Portal Capture",
        "tactic": "collection"
      },
      {
        "platform": "macOS",
        "tid": "T1056.003",
        "technique": "Web Portal Capture",
        "tactic": "credential-access"
      },
      {
        "platform": "Windows",
        "tid": "T1056.003",
        "technique": "Web Portal Capture",
        "tactic": "collection"
      },
      {
        "platform": "Windows",
        "tid": "T1056.003",
        "technique": "Web Portal Capture",
        "tactic": "credential-access"
      },
      {
        "platform": "Linux",
        "tid": "T1071.001",
        "technique": "Web Protocols",
        "tactic": "command-and-control"
      },
      {
        "platform": "macOS",
        "tid": "T1071.001",
        "technique": "Web Protocols",
        "tactic": "command-and-control"
      },
      {
        "platform": "Windows",
        "tid": "T1071.001",
        "technique": "Web Protocols",
        "tactic": "command-and-control"
      },
      {
        "platform": "Office 365",
        "tid": "T1550.004",
        "technique": "Web Session Cookie",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Office 365",
        "tid": "T1550.004",
        "technique": "Web Session Cookie",
        "tactic": "lateral-movement"
      },
      {
        "platform": "SaaS",
        "tid": "T1550.004",
        "technique": "Web Session Cookie",
        "tactic": "defense-evasion"
      },
      {
        "platform": "SaaS",
        "tid": "T1550.004",
        "technique": "Web Session Cookie",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Linux",
        "tid": "T1505.003",
        "technique": "Web Shell",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1505.003",
        "technique": "Web Shell",
        "tactic": "persistence"
      },
      {
        "platform": "macOS",
        "tid": "T1505.003",
        "technique": "Web Shell",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1059.003",
        "technique": "Windows Command Shell",
        "tactic": "execution"
      },
      {
        "platform": "Windows",
        "tid": "T1222.001",
        "technique": "Windows File and Directory Permissions Modification",
        "tactic": "defense-evasion"
      },
      {
        "platform": "Windows",
        "tid": "T1546.003",
        "technique": "Windows Management Instrumentation Event Subscription",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1546.003",
        "technique": "Windows Management Instrumentation Event Subscription",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1021.006",
        "technique": "Windows Remote Management",
        "tactic": "lateral-movement"
      },
      {
        "platform": "Windows",
        "tid": "T1543.003",
        "technique": "Windows Service",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1543.003",
        "technique": "Windows Service",
        "tactic": "privilege-escalation"
      },
      {
        "platform": "Windows",
        "tid": "T1547.004",
        "technique": "Winlogon Helper DLL",
        "tactic": "persistence"
      },
      {
        "platform": "Windows",
        "tid": "T1547.004",
        "technique": "Winlogon Helper DLL",
        "tactic": "privilege-escalation"
      }
    ]
  },
  "uniques_techniques": [
    "T1001",
    "T1003",
    "T1005",
    "T1006",
    "T1007",
    "T1008",
    "T1010",
    "T1011",
    "T1012",
    "T1014",
    "T1016",
    "T1018",
    "T1020",
    "T1021",
    "T1025",
    "T1026",
    "T1027",
    "T1029",
    "T1030",
    "T1033",
    "T1036",
    "T1037",
    "T1039",
    "T1040",
    "T1041",
    "T1046",
    "T1047",
    "T1048",
    "T1049",
    "T1051",
    "T1052",
    "T1053",
    "T1055",
    "T1056",
    "T1057",
    "T1061",
    "T1062",
    "T1068",
    "T1069",
    "T1070",
    "T1071",
    "T1072",
    "T1074",
    "T1078",
    "T1080",
    "T1082",
    "T1083",
    "T1087",
    "T1090",
    "T1091",
    "T1092",
    "T1095",
    "T1098",
    "T1102",
    "T1104",
    "T1105",
    "T1106",
    "T1108",
    "T1110",
    "T1111",
    "T1112",
    "T1113",
    "T1114",
    "T1115",
    "T1119",
    "T1120",
    "T1123",
    "T1124",
    "T1125",
    "T1127",
    "T1129",
    "T1132",
    "T1133",
    "T1134",
    "T1135",
    "T1136",
    "T1137",
    "T1140",
    "T1149",
    "T1153",
    "T1176",
    "T1185",
    "T1187",
    "T1189",
    "T1190",
    "T1195",
    "T1197",
    "T1199",
    "T1200",
    "T1201",
    "T1202",
    "T1203",
    "T1204",
    "T1207",
    "T1210",
    "T1211",
    "T1212",
    "T1213",
    "T1216",
    "T1217",
    "T1218",
    "T1219",
    "T1220",
    "T1221",
    "T1222",
    "T1480",
    "T1482",
    "T1484",
    "T1485",
    "T1486",
    "T1489",
    "T1490",
    "T1491",
    "T1495",
    "T1496",
    "T1497",
    "T1498",
    "T1499",
    "T1505",
    "T1518",
    "T1525",
    "T1526",
    "T1528",
    "T1529",
    "T1530",
    "T1531",
    "T1534",
    "T1535",
    "T1537",
    "T1538",
    "T1539",
    "T1542",
    "T1543",
    "T1546",
    "T1547",
    "T1548",
    "T1550",
    "T1552",
    "T1554",
    "T1555",
    "T1556",
    "T1557",
    "T1558",
    "T1559",
    "T1560",
    "T1561",
    "T1562",
    "T1563",
    "T1564",
    "T1565",
    "T1566",
    "T1567",
    "T1568",
    "T1569",
    "T1570",
    "T1571",
    "T1572",
    "T1573",
    "T1574",
    "T1578"
  ],
  "uniques_subtechniques": [
    "T1001.001",
    "T1001.002",
    "T1001.003",
    "T1003.001",
    "T1003.002",
    "T1003.003",
    "T1003.004",
    "T1003.005",
    "T1003.006",
    "T1003.007",
    "T1003.008",
    "T1011.001",
    "T1021.001",
    "T1021.002",
    "T1021.003",
    "T1021.004",
    "T1021.005",
    "T1021.006",
    "T1027.001",
    "T1027.002",
    "T1027.003",
    "T1027.004",
    "T1027.005",
    "T1036.001",
    "T1036.002",
    "T1036.003",
    "T1036.004",
    "T1036.005",
    "T1036.006",
    "T1037.001",
    "T1037.002",
    "T1037.003",
    "T1037.004",
    "T1037.005",
    "T1048.001",
    "T1048.002",
    "T1048.003",
    "T1052.001",
    "T1053.001",
    "T1053.002",
    "T1053.003",
    "T1053.004",
    "T1053.005",
    "T1055.001",
    "T1055.002",
    "T1055.003",
    "T1055.004",
    "T1055.005",
    "T1055.008",
    "T1055.009",
    "T1055.011",
    "T1055.012",
    "T1055.013",
    "T1055.014",
    "T1056.001",
    "T1056.002",
    "T1056.003",
    "T1056.004",
    "T1059.001",
    "T1059.002",
    "T1059.003",
    "T1059.004",
    "T1059.005",
    "T1059.006",
    "T1059.007",
    "T1069.001",
    "T1069.002",
    "T1069.003",
    "T1070.001",
    "T1070.002",
    "T1070.003",
    "T1070.004",
    "T1070.005",
    "T1070.006",
    "T1071.001",
    "T1071.002",
    "T1071.003",
    "T1071.004",
    "T1074.001",
    "T1074.002",
    "T1078.001",
    "T1078.002",
    "T1078.003",
    "T1078.004",
    "T1087.001",
    "T1087.002",
    "T1087.003",
    "T1087.004",
    "T1090.001",
    "T1090.002",
    "T1090.003",
    "T1090.004",
    "T1098.001",
    "T1098.002",
    "T1098.003",
    "T1098.004",
    "T1102.001",
    "T1102.002",
    "T1102.003",
    "T1110.001",
    "T1110.002",
    "T1110.003",
    "T1110.004",
    "T1114.001",
    "T1114.002",
    "T1114.003",
    "T1127.001",
    "T1132.001",
    "T1132.002",
    "T1134.001",
    "T1134.002",
    "T1134.003",
    "T1134.004",
    "T1134.005",
    "T1136.001",
    "T1136.002",
    "T1136.003",
    "T1137.001",
    "T1137.002",
    "T1137.003",
    "T1137.004",
    "T1137.005",
    "T1137.006",
    "T1195.001",
    "T1195.002",
    "T1195.003",
    "T1204.001",
    "T1204.002",
    "T1205.001",
    "T1213.001",
    "T1213.002",
    "T1216.001",
    "T1218.001",
    "T1218.002",
    "T1218.003",
    "T1218.004",
    "T1218.005",
    "T1218.007",
    "T1218.008",
    "T1218.009",
    "T1218.010",
    "T1218.011",
    "T1222.001",
    "T1222.002",
    "T1480.001",
    "T1491.001",
    "T1491.002",
    "T1497.001",
    "T1497.002",
    "T1497.003",
    "T1498.001",
    "T1498.002",
    "T1499.001",
    "T1499.002",
    "T1499.003",
    "T1499.004",
    "T1505.001",
    "T1505.002",
    "T1505.003",
    "T1518.001",
    "T1542.001",
    "T1542.002",
    "T1542.003",
    "T1543.001",
    "T1543.002",
    "T1543.003",
    "T1543.004",
    "T1546.001",
    "T1546.002",
    "T1546.003",
    "T1546.004",
    "T1546.005",
    "T1546.006",
    "T1546.007",
    "T1546.008",
    "T1546.009",
    "T1546.010",
    "T1546.011",
    "T1546.012",
    "T1546.013",
    "T1546.014",
    "T1546.015",
    "T1547.001",
    "T1547.002",
    "T1547.003",
    "T1547.004",
    "T1547.005",
    "T1547.006",
    "T1547.007",
    "T1547.008",
    "T1547.009",
    "T1547.010",
    "T1547.011",
    "T1548.001",
    "T1548.002",
    "T1548.003",
    "T1548.004",
    "T1550.001",
    "T1550.002",
    "T1550.003",
    "T1550.004",
    "T1552.001",
    "T1552.002",
    "T1552.003",
    "T1552.004",
    "T1552.005",
    "T1552.006",
    "T1553.001",
    "T1553.002",
    "T1553.003",
    "T1553.004",
    "T1555.001",
    "T1555.002",
    "T1555.003",
    "T1556.001",
    "T1556.002",
    "T1556.003",
    "T1557.001",
    "T1558.001",
    "T1558.002",
    "T1558.003",
    "T1559.001",
    "T1559.002",
    "T1560.001",
    "T1560.002",
    "T1560.003",
    "T1561.001",
    "T1561.002",
    "T1562.001",
    "T1562.002",
    "T1562.003",
    "T1562.004",
    "T1562.006",
    "T1562.007",
    "T1563.001",
    "T1563.002",
    "T1564.001",
    "T1564.002",
    "T1564.003",
    "T1564.004",
    "T1564.005",
    "T1564.006",
    "T1565.001",
    "T1565.002",
    "T1565.003",
    "T1566.001",
    "T1566.002",
    "T1566.003",
    "T1567.001",
    "T1567.002",
    "T1568.001",
    "T1568.002",
    "T1568.003",
    "T1569.001",
    "T1569.002",
    "T1573.001",
    "T1573.002",
    "T1574.001",
    "T1574.002",
    "T1574.004",
    "T1574.005",
    "T1574.006",
    "T1574.007",
    "T1574.008",
    "T1574.009",
    "T1574.010",
    "T1574.011",
    "T1574.012",
    "T1578.001",
    "T1578.002",
    "T1578.003",
    "T1578.004"
  ],
  "stats": {
    "count_revoked_techniques": 136,
    "count_active_techniques": 160,
    "count_active_subtechniques": 272,
    "count_malwares": 351,
    "count_adversaries": 109,
    "count_tools": 61,
    "count_platforms": 9,
    "count_tactics": 12,
    "count_datasources": 58
  }
}
```