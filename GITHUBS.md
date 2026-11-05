# Which GitHub is which

Company code and lookalike orgs, as listed on 5 November 2026. A new repo in a real org matters. A repo that only shares a name does not.

## Companies

| Org | Why it is in this note | Public repos then |
| --- | --- | --- |
| [SpaceExplorationTechnologies](https://github.com/SpaceExplorationTechnologies) | Blog `spacex.com`, created 2014. Holds the Starlink Wi-Fi GPL tree, `enterprise-api`, and forks of u-boot, linux, linuxptp, sssd. | 6 |
| [teslamotors](https://github.com/teslamotors) | "Tesla, Inc.", blog `tesla.com`. Vehicle command, fleet telemetry, TTPoE, vendor kernels, energy-adjacent models. | 67 |
| [xai-org](https://github.com/xai-org) | Displayed as SpaceXSI, blog `x.ai`. Grok-1, the current For You code, API SDKs, grok-build. | 12 |
| [twitter](https://github.com/twitter) | "X (fka Twitter)", blog `opensource.x.com`. Community Notes is still maintained. The older recommendation drop lives here. | 102 |
| [xdevplatform](https://github.com/xdevplatform) | X developer samples. A few are current (`xurl`, `chat-xdk`, `docs`). | 117 |
| [neuralink](https://github.com/neuralink) | Separate medical-device company. Four repos. See [adjacent](branches/15-adjacent.md). | 4 |
| [boringcompany](https://github.com/boringcompany) | One app, last push 2019. | 1 |

SpaceX has no large flight-software org. Falcon, Dragon, and Starship guidance are not in the six-repo org. Terafab and Starmind had no software org on this date.

Repos that show how a branch actually works:

| Repo | What it is |
| --- | --- |
| [SpaceExplorationTechnologies/enterprise-api](https://github.com/SpaceExplorationTechnologies/enterprise-api) | Starlink device and business API surface. Last push in this pass: August 2025. |
| [SpaceExplorationTechnologies/starlink-wifi](https://github.com/SpaceExplorationTechnologies/starlink-wifi) | GPL router stack. The application payload is not in the box. |
| [teslamotors/vehicle-command](https://github.com/teslamotors/vehicle-command) | The car executes a command that matches a key in its keychain. |
| [teslamotors/fleet-telemetry](https://github.com/teslamotors/fleet-telemetry) | The owner chooses the server. The car pushes records there. |
| [teslamotors/ttpoe](https://github.com/teslamotors/ttpoe) | Layer-2 Ethernet transport aimed at Dojo-class links. |
| [teslamotors/nempy](https://github.com/teslamotors/nempy) | A model of Australian electricity dispatch. |
| [xai-org/x-algorithm](https://github.com/xai-org/x-algorithm) | For You ranking code. Apache-2.0. |
| [twitter/the-algorithm](https://github.com/twitter/the-algorithm) | The earlier public drop of that ranking code. |
| [twitter/communitynotes](https://github.com/twitter/communitynotes) | Community Notes algorithm and docs. |
| [xai-org/grok-1](https://github.com/xai-org/grok-1) | Grok-1 weights and code, 2024, Apache-2.0. Later Grok models are not this repo. |
| [kaspanet/rusty-kaspa](https://github.com/kaspanet/rusty-kaspa) | The node. Not a SpaceX or Tesla repo. |
| [kaspanet/kips](https://github.com/kaspanet/kips) | The rules. |
| [kaspanet/kccs](https://github.com/kaspanet/kccs) | The conventions. |

## Lookalikes

| GitHub | What it actually is |
| --- | --- |
| [Starlink](https://github.com/Starlink) | Astronomy software. Blog `starlink.ac.uk`. Not the satellite internet. |
| [tesla](https://github.com/tesla) | Old Maven and Eclipse plugins. Not Tesla, Inc. |
| [spacex](https://github.com/spacex) | A personal account. Not the company. |
| [spacexcorp](https://github.com/spacexcorp) | Unrelated forks. Not used. |
| [r-spacex/SpaceX-API](https://github.com/r-spacex/SpaceX-API) | Community project. Archived 6 June 2026. The README says it is not affiliated with Space Exploration Technologies Corp. |
| [SpaceXpanse](https://github.com/SpaceXpanse) | An unrelated project. |

Reporting in 2026 describes SpaceX as having brought xAI, and with it X, into the company, and describes Terafab as a project with Tesla. Tesla is still a separate listed company. Neuralink and The Boring Company sit beside that pair because of the founder, not because their repos are inside `teslamotors`. This page did not read an SEC filing. If a filing and a blog disagree, use the filing.
