# Optimus

Optimus is Tesla's humanoid robot program. Public repositories do not describe the control stack. The machines, if deployed, will have safety logs, video, and a fleet cloud. Those files are sensitive in a factory and more sensitive in a home.

## 1. Exposure

A robotics cloud account is a single switch for a fleet: tasking, logs, and model updates. A government that can compel Tesla can order a fleet paused in a jurisdiction, or order logs preserved, or order logs destroyed. Video of workers and homes makes a public chain the wrong archive. Publishing a log to make it "uncensorable" would harm the people in the frame.

Export rules on actuators, compute, and software can hold a shipment. The chain does not clear customs.

The improvement is about **version and incident hashes**, the same narrow object as the cars, before a large installed base exists. Exploring it early is cheaper than retrofitting a fleet that already logged everything to one tenant.

## 2. Worth exploring

Seal the hash of a software image when it is released to robots, and the hash of a safety-incident pack when that pack is shared with an insurer, a workplace regulator, or an internal safety board. The video and the joint traces stay in the pack, off-chain. A later claim that "the pack was different" can be checked. A claim that "there was no pack" can be checked if the digest was broadcast.

Use the owner's chosen store for the bytes, as fleet telemetry already does for cars. Use the rail only as the clock.

Do not build a public market for robot tasking on this chain. Tasking is the operator's liability.

## 3. While miners mine

The image digest and the incident digest remain if the robotics tenant is closed or if one party deletes their copy. Miners do not receive the video.

The robot still obeys the software it is running and the company that signs updates. A pause order served on Tesla still pauses the service Tesla operates.

## What this rail does not do

It does not balance a robot, store camera rolls, or create a permissionless command channel for a machine that can hurt people.

## Detail

The image and the incident pack are two calls. The pack stays off the payload. Eighty-one bytes cannot hold a video, and the primary field is a hash even when the payload is full.

```rust
let image = kaspa_seal::seal_file(
    kaspa_seal::Branch::Optimus,
    kaspa_seal::Kind::WeightOrImage,
    software_image,
);
let incident = kaspa_seal::seal_file(
    kaspa_seal::Branch::Optimus,
    kaspa_seal::Kind::Inspection,
    incident_pack_shared_with_the_board,
);
```

Hashing the pack commits to it. Hand the pack to the insurer or the safety board through the channel they already use. Do not put the camera roll in the transaction. A public chain plus a leaked roll would let anyone check that the roll is the one that was sealed. That is a reason to choose the hash input carefully, not a reason to seal everything.

Tasking is not a kind in this crate. There is no `Kind` for a command. [KIP-16](https://github.com/kaspanet/kips/blob/master/kip-0016.md) would still only check a predefined proof tag, and v2.1.0 implements two. A proof tag is not a torque command. The robot runs the image it has, under the key that signed the update.

The digest function and the tests against the node's incremental `PayloadDigest` vectors are in [the code notes](../code/README.md).
