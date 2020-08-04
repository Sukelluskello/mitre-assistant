# mitre-assistant

A custom, more useful, and much cooler MITRE-CTI-CLIENT.

<br/>

![image](https://user-images.githubusercontent.com/11415591/89246990-4e367500-d5da-11ea-9519-d94994fecdfc.png)

<br/>

```bash
# Assumes you have installed the rust tool chain
# and that you have the `cargo` package manager
#
# Preferably use rust stable channel
#
$> cargo install mitre-assistant
```
<br/>

<br/>

<hr>

## W.I.P - Status
- [x] Mitre Enterprise Matrix
- [ ] Mitre Mobile Matrix
- [ ] Mitre Pre-Attack Matrix
- [x] Linux - 64bit
- [x] MacOS - 64bit
- [x] Windows - 64bit
- [ ] Data Interchange Format
   - [ ] CSV
   - [ ] JSON
- [ ] Exports
   - [ ] CSV
   - [ ] JSON
   - [ ] Rich Web

<hr>

<br/>
<br/>

# Getting Started
You got 3 ways to start using this `bad-boy`:

**1.** You can go to the releases section, download the pre-compiled binary for your os. Note:  I only provide Debian on Linux

**2.** If you already have rust stable toolchain installed, then simply use `cargo install mitre-assistant`

**3.** Or, if you just love building from source, follow the instructions in the `build from source section` below.


<br/>
<br/>

## Releases - Binaries
Head over to the [releases section](https://github.com/dfirence/mitre-assistant/releases/) and download the binary for your OS.  However, note, I am only supporting binaries for **64 bit versions** of:

* MacOS
* Debian
* Windows

<br/>

## Build From Source
If you use a different Linux distro, install the rust toolchain, preferably the stable channel, and follow these steps:

### Step 1 - Clone this repo

```bash
$> git clone https://github.com/dfirence/mitre-assistant.git
```

<br/>

### Step 2 - Navigate into the repo

```bash
$> cd mitre-assistant
```

<br/>

### Step 3 - Build/Compile

```bash
$> cargo build --release
```
<br/>

### Step 4 - Move your fresh binary to a system path

In this step, if you wanna call the executable from anywhere, add it to your system path or executable path - i.e., /usr/bin
```bash
$> sudo mv /target/release/mitre-assistant /usr/bin
```
<br/>

<br/>
<br/>

## Why are you doing this?
I work in the Security industry for a provider, my work hinges a lot on this resource from The Mitre Corporation.  At some point, if you are like me, you will observe the poor and ridiculous amount of time that is needed to create custom datasets from that resource and collaborate across teams to get into serious work.  This helps me not waste time on silly things - i.e., clicking on some website, or asking important questions so I can incorporate the matrix into some form of tactical plans to defend my network, or support new strategies while working with others.

<br/>

## Why not use other existing community tools for this?
I have seen them, used them, and appreciate those that are writing their own. In the end, I am not gonna wait for anyone to do things the way I need them.

<br/>
<br/>

# **Usage**

This is a modular tool. The main concept of using this tool is:

```text

            (1)                      (2)                       (3)
             |                        |                         |
             |                        |                         |
        [ Extract ]-------------[ Transform ]---------------[ Load ]
             |                        |                         |
             |                        |                         |
             |                        |                         |
             v                        v                         v
       Download A Matrix         Baseline The Matrix        Search - Ask your question
```
<br/>
<br/>

## **Help Menu**
Building from the above concept, let's get into using this bad-boy.

<br/>

```bash
cdiaz@[mitre-assistant]
 >> ./target/release/mitre-assistant -h



mitre-assistant v.0.0.1
carlos diaz | @dfirence

Mitre Attack Assistant

        A more useful utility for the ATT&CK Matrix

USAGE:
    mitre-assistant [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    baseline    Parse a Matrix into comprehensive insights
    download    Download a Matrix From The Mitre CTI Repo
    help        Prints this message or the help of the given subcommand(s)
    search      Search The Baseline
```
<br/>
<br/>

# *Download*
Use the `download` subcommand to get started, you can specific which matrix to download by using any of the keywords: `enterprise` or `mobile` or `pre-attack`

<br/>

```bash
# Assumes you want to download the `enterprise` matrix
#
$> mitre-assistant download -m enterprise


# Output
===========================================================================================

Downlading Matrix : enterprise
Downloading From  : https://raw.githubusercontent.com/mitre/cti/master/enterprise-attack/enterprise-attack.json

===========================================================================================
        |__(+) New File To Be Created: /Users/alice/.mitre-assistant/matrixes/enterprise.json
```
<br/>
<br/>

# *Baseline*
Use the `baseline` subcommand after you download your matrix to create the custom database that is required before you conduct your searches.

You baseline a matrix with any of the keywords:  `enterprise` or `mobile` or `pre-attack`

<br/>

```bash
$> mitre-assistant baseline -m enterprise


#Output
/Users/alice/.mitre-assistant/matrixes/enterprise.json
  |__(+) New File To Be Created: /Users/alice/.mitre-assistant/baselines/baseline-enterprise.json
```
<br/>
<br/>

# *Search*
Now you are ready to search your matrix.

You have to tell the `search subcommand` which matrix it is going to work with by using:

* the `-m` parameter followed by the name of the matrix 
* the `-t` parameter to provide your search term.

<br/>
<br/>
## *Search Terms*

|TERM|MATRIX|PURPOSE|
|----|------|-------|
|`stats`|*enterprise*|An overview of `uniq` counts and `total` counts of key data elements|

<br/>
<br/>

## *Searching The Enterprise Matrix For An Overview Stats Summary*
You use the keyword `stats` in your search term, like this

```bash
# Assumed you want the summary of items in the matrix
#
$> mitre-assistant search -m enterprise -t "stats"
```
<br/>
<br/>

|UNIQUES & TOTALS|
|-----------------|
|![image](https://user-images.githubusercontent.com/11415591/89135007-ff181380-d4f7-11ea-8b70-97ae6421d3d1.png)|



<br/>
<br/>


| TECHNIQUES | SUBTECHNIQUES|
|------------|--------------|
|![image](https://user-images.githubusercontent.com/11415591/89134982-e3147200-d4f7-11ea-9e31-f985500d2ee3.png)|![image](https://user-images.githubusercontent.com/11415591/89134971-d263fc00-d4f7-11ea-992c-39730ead0421.png)|



<br/>
<br/>

## *Searching The Enterprise Matrix For A Single Technique By ID*


<br/>

```bash
# Assumes you want to search/query the enterprise matrix
# All terms must be enclosed by double-quotes
#
$> mitre-assistant search -m enterprise -t "t1021"
```
<br/>

![image](https://user-images.githubusercontent.com/11415591/89109722-cf8edb80-d411-11ea-82b5-3a4dde2d90b1.png)

<br/>

## *Searching The Enterprise Matrix For Many Techniques By ID*
Cool, now you just have to add a comma `,` in your term and launch it again, dead-simple!

<br/>

```bash
# Assumes you want to search for techniques:  T1021 & T1048
#
$> mitre-assistant search -m enterprise -t "t1021,t1048"

```

<br/>

![image](https://user-images.githubusercontent.com/11415591/89109703-ae2def80-d411-11ea-9268-ab7f42527386.png)

<br/>

## *Searching The Enterprise Matrix & Displaying The Subtechniques*
Another cool thing here is display the `subtechniques` for your query by using:

* the `-s` flag after your query

<br/>

```bash
# Assumes you want to see the Subtechniques for T1021
$> mitre-assistant search -m enterprise -t "t1021" -s
```
<br/>

![image](https://user-images.githubusercontent.com/11415591/89109790-69568880-d412-11ea-9869-325a35d7de13.png)

<br/>

## *Searching For The Revoked Techniques*
Revoked techniques seem to be those that are discontinued and re-arranged now into subtechniques.  You can search for the ones `revoked` in the matrix by using a keyword in your search term:

* the `-t` parameters with the term `revoked`

<br/>

```bash
# Assumes you want to see the revoked techniques
#
$> mitre-assistant search -m enterprise -t "revoked"
```
<br/>

![image](https://user-images.githubusercontent.com/11415591/89109865-074a5300-d413-11ea-87e9-5fadeb569e84.png)

<br/>
<br/>

## *Searching For The Datasources*

```text
Protip:

1. Do not follow Mitre blindly, you need to curate their content
and organize it.

Example:

1. DLL Monitoring & Loaded DLLs

Mitre currently has these two datasources, what does this mean?

To me in the security Space, there's only one source, not two.
```
<br/>

Datasources are a non-concrete description by Mitre that seems to suggest the context of evidence needed to be successful at pursuing visibility or detection capabilities for the given technique. This query gets you the datasources as provided by Mitre in their CTI github

* the `-t` parameters with the term `datasources`

<br/>

```bash
# Assumes you want to see the Datasources
# for the enterprise matrix
#
$> mitre-assistant -m enterprise -t "datasources"
```

<br/>

## TODO: ADD Screenshot here

<br/>
<br/>

## *Searching For The Platforms*
Platforms are the relevant operating systems where a technique is exercised or abused by an adversary. To get the platforms in the enterprise matrix use the keyword `platforms`.

* the `-t` parameters with the term `platforms`

<br/>

```bash
# Assumes you want to see the Datasources
# for the enterprise matrix
#
$> mitre-assistant -m enterprise -t "platforms"
```

<br/>

## TODO: ADD Screenshot here

<br/>

## *Searching For Edge Cases:  Techniques Without a Datasource*
This is the edge-case that drove to create this tool for myself.  I found someone's tool incorrectly parsed the matrix and I needed to report to my management the plan of action based on data sources.  This is very important for practitioners who leverage the matrix for real world tactical operations.

Reference this example:  [NO_DATA_SOURCE_SAMPLE](https://user-images.githubusercontent.com/11415591/88487153-a58c7380-cf50-11ea-8547-e03a6b7a9185.png)

Use the keyword `nodatasources` to obtain a list of active techniques that may not have an assigned datasource by Mitre.

* the `-t` parameters with the term `nodatasources`

<br/>

```bash
# Assumes you want to see the Datasources
# for the enterprise matrix
#
$> mitre-assistant -m enterprise -t "nodatasources"
```

<br/>

## TODO: ADD Screenshot here

<br/>
<br/>

# **Statistical Stuff**
As I mentioned, my work with this matrix is at the provider level, I have to devise coverage plans, or brainstorming workshops with my fellow blue-teamers to understand what an emulation plan means in terms of effort, engineering for new content and consequently sizing our systems to increase our visibility and detection needs.

These experiments were very useful to me a couple of years ago as I started learning about the Mitre ATT&CK matrixes.

<br/>

## TODO: Awesome Stuff here


<br/>
<br/>
<br/>

# Kudos - RUSTACEANS
Many super kudos, to the amazing RUST Community, for their warm embrace of everyone that wants the journey.  Seemingly, to all of the super creators of loved tools from python being ported into rust.

## TODO - Thank Crates Contributions