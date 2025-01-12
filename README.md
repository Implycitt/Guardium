<br />
<div align="center">
  <h3 align="center">Guardium</h3>

  <p align="center">
    Tower defense game written entirely in Rust.
    <br />
    <br />
    <a href="https://github.com/Implycitt/Guardium">View Demo</a>
  </p>
</div>

<br />

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#Screenshots">Screenshots</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>


<!-- ABOUT THE PROJECT -->
## About The Project

![Product Name Screen Shot][product-screenshot]

A strategy tower defense game create in rust using the bevy game engine. The project is my AP Computer Science Principles final.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


### Built With

* [![Rust]][rust-url]
* [![Bevy]][bevy-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

To compile the project locally, ensure that [rust](https://www.rust-lang.org/tools/install) is installed.
* You can check cargo version with:  
  ```sh
  cargo --version
  ```

## Installation

### Releases tab:

1. download executable from the [Releases](https://github.com/Implycitt/Guardium/releases) tab
2. Run the Executable

### Compiling and running localy:
1. Clone the repo
   ```sh
   git clone https://github.com/Implycitt/Guardium.git
   ```
2. Build using cargo
   ```sh
   //only builds the program
   cargo build
   //builds and runs
   cargo run
   ```
<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- SCREENSHOTS -->
## Screenshots

![First GIF][firstgif-screenshot]
![Update][update-screenshot]
![First Build][fb-screenshot]

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

Roadmap to be updated with some specifics maybe and hopefully a full tree of features to add
- [x] enemies that spawn randomly around tower
    - [x] find shortest path to tower
- [x] tower that shoots at nearest enemy
- [ ] wave system for rounds
- [ ] Resources
    - [ ] Inventory and resource management
    - [ ] Timer for collection
    - [ ] Resources allow different upgrades
- [ ] Multiplayer
    - [ ] Hosting and joining
    - [ ] Chatting
- [ ] Procedural terrain/biome generation
    - [ ] Players spawn in random biomes
    - [ ] Each biome has different resources
- [ ] UI 
- [ ] Menu; main and options
- [ ] Music

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
[contributors-shield]: https://img.shields.io/github/contributors/Implycitt/Guardium.svg?style=for-the-badge
[contributors-url]: https://github.com/Implycitt/Guardium/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Implycitt/Guardium.svg?style=for-the-badge
[forks-url]: https://github.com/Implycitt/Guardium/network/members
[stars-shield]: https://img.shields.io/github/stars/Implycitt/Guardium.svg?style=for-the-badge
[stars-url]: https://github.com/Implycitt/Guardium/stargazers
[issues-shield]: https://img.shields.io/github/issues/Implycitt/Guardium.svg?style=for-the-badge
[issues-url]: https://github.com/Implycitt/Guardium/issues
[license-shield]: https://img.shields.io/github/license/Implycitt/Guardium.svg?style=for-the-badge
[license-url]: https://github.com/Implycitt/Guardium/blob/master/LICENSE.txt
[product-screenshot]: assets/github/firstGIF.gif
[Rust]: https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324
[rust-url]: https://www.rust-lang.org/
[bevy]: https://img.shields.io/badge/Bevy-232326?logo=bevy&logoColor=fff&style=flat
[bevy-url]: https://bevyengine.org/
[fb-screenshot]: assets/github/FirstBuild.png
[update-screenshot]: assets/github/Update23.03.24.png 
[firstgif-screenshot]: assets/github/firstGIF.gif
