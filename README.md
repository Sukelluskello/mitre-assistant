# mitre-assistant

A custom, more useful, and cooler MITRE-CTI-CLIENT.

<br/>

![image](https://user-images.githubusercontent.com/11415591/89109095-36a99180-d40c-11ea-95ef-da114f6ea7ca.png)

<br/>

## W.I.P - Status
- [x] Mitre Enterprise Matrix
- [ ] Mitre Mobile Matrix
- [ ] Mitre Pre-Attack Matrix
- [x] Linux
- [x] MacOS
- [ ] Windows
<br/>

## Why are you doing this?
I work in the Security industry for a provider, my work hinges a lot on this resource from The Mitre Corporation.  At some point, if you are like me, you will observe the poor and ridiculous amount of time that is needed to create custom datasets from that resource and collaborate across teams to get into serious work.  This helps me not waste time on silly things - i.e., clicking on some website, or asking important questions so I can incorporate the matrix into some form of tactical plans to defend my network, or support new strategies while working with others.

## Why not use other existing community tools for this?
I have seen them, used them, and appreciate those that are writing their own. In the end, I am not gonna wait for anyone to do things the way I need them.

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

Building from the above concept, let's get into using this bad-boy.

## **Help Menu**

```bash
cdiaz@[mitre-assistant]
 >> ./target/release/mitre-assistant -h



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

## *Searching The Enterprise Matrix For a Stats Summary*
You use the keyword `stats` in your search term, like this

```bash
# Assumed you want the summary of items in the matrix
#
$> mitre-assistant search -m enterprise -t "stats"
```
<br/>

![image](https://user-images.githubusercontent.com/11415591/89109965-ef270380-d413-11ea-9c4b-c780f2927fe0.png)

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

## *Searching The Enterprise Matrix For A Many Techniques By ID*
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