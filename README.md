<a name="readme-top"></a>


<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.comosm6495/scanbom/">
  </a>

  <h3 align="center">ScanBOM</h3>

  <p align="center">
    A CLI tool to parses SBOM files and scan all dependencies with Semgrep
    <br />
    <a href="https://github.comosm6495/scanbom/">View Demo</a>
    ·
    <a href="https://github.comosm6495/scanbom/issues">Report Bug</a>
    ·
    <a href="https://github.comosm6495/scanbom/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#usage">Usage</a>
      <ul>
        <li><a href="#examples">Examples</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#installing-the-latest-version">Installing the latest version</a></li>
        <li><a href="#installing-from-source">Installing from source</a></li>
      </ul>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT -->
## Usage

```
Parse SBOM files and scan all dependencies with Semgrep

Usage: scanbom [OPTIONS] <file_path>

Arguments:
  <file_path>  Path to the SBOM file

Options:
  -c, --clean-json                 Output simplified JSON format
  -j, --json-input                 Use JSON input format
  -t, --timer                      Measure elapsed time
  -q, --quiet                      Only print output, with no loading bar
  -o, --output-type <output_type>  Specify the output format [default: text] [possible values: json, semgrep, emacs, gitlab-sast, gitlab-secrets, junit-xml, sarif, text, vim]
  -h, --help                       Print help
  -V, --version                    Print version
```

Simply pass in an SBOM in SPDX format and ScanBOM will clone down all the dependencies and scan them with Semgrep. For any open source projects, you can get an SBOM in this format from GitHub by going to the `Insights` tab of your repository and then going down to the `Dependency Graph` and clicking the `Export SBOM` button on the top right.

### Examples
- Scan an SPDX SBOM file `sbom.json` stored in the examples directory of the repo. The example `sbom.json` file there is an SBOM generated for the `lodash` repository: https://github.com/lodash/lodash
```
scanbom "./examples/sbom.json"
```
- Scan a custom list in the following JSON format:
```json
{
  "packages": [ 
    {
      "name": "lodash",
      "version": "4.0.0"
    },
    {
      "name": "mongoose",
      "version": "8.2.3"
    }
  ]
}
```
There is an example `input.json` file in the examples directory that can be scanned with:
```
scanbom -j "./examples/input.json"
```
- Output raw JSON from the Semgrep scan and pipe it to other CLI tools or programs
```
scanbom -qo json "./examples/sbom.json"
```

<!-- GETTING STARTED -->
## Getting Started

### Installing the latest version
You can use download a pre-built binary directly from the latest release: https://github.com/osm6495/scanbom/releases

1. Select the latest version at the top of the page and open the `Assets` section
2. Download the file that applies for your system
3. (Optional) Move the binary to your `/usr/bin` directory for Linux and Mac or `C:\Program Files` for Windows. This will allow you to use the `scanbom` command without directly calling the binary or having the source code.


### Installing from Source

_Below is an example of how you can instruct your audience on installing and setting up your app. This template doesn't rely on any external dependencies or services._

1. Install Rust: [http://rust-lang.org/](http://rust-lang.org/)
2. Clone the repo
  ```sh
  git clone https://github.com/osm6495/scanbom
  cd scanbom
  ```
3. Build the binary
  ```sh
  cargo build --release
  ```
4. Run the program
  ```sh
  ./target/release/sbom -h
  ```
5. (Optional) Move the binary to your `/usr/bin` directory for Linux and Mac or `C:\Program Files` for Windows. This will allow you to use the `scanbom` command without directly calling the binary or having the source code.
  ```sh
  sudo mv ./target/release/scanbom /usr/bin/scanbom
  ```

<!-- ROADMAP -->
## Roadmap

- [ ] Add ability to include custom Semgrep rules

See the [open issues](https://github.com/osm6495/scanbom/issues) for a full list of proposed features (and known issues).




<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request




<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.




<!-- CONTACT -->
## Contact

Owen McCarthy - contact@owen.biz

<!-- ACKNOWLEDGEMENT -->
## Acknowledgements
- [Semgrep Docs](https://semgrep.dev/docs/)
- [About SPDX](https://spdx.dev/about/overview/)
- [README Template](https://github.com/othneildrew/Best-README-Template)

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/osm6495/scanbom.svg?color=orange
[contributors-url]: https://github.com/osm6495/scanbom/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/osm6495/scanbom.svg?style=flat&color=orange
[forks-url]: https://github.com/osm6495/scanbom/network/members
[stars-shield]: https://img.shields.io/github/stars/osm6495/scanbom.svg?style=flat&color=orange
[stars-url]: https://github.com/osm6495/scanbom/stargazers
[issues-shield]: https://img.shields.io/github/issues/osm6495/scanbom.svg?color=orange
[issues-url]: https://github.com/osm6495/scanbom/issues
[license-shield]: https://img.shields.io/github/license/osm6495/scanbom.svg?color=orange
[license-url]: https://github.com/osm6495/scanbom/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?color=blue&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/owen-mccarthy-060827192/
