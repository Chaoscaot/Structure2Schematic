# Structure2Schematic
### Convert a Minecraft structure file to a schematic file

---

## WARNING: This project is still in development

#### Features:
- [x] Convert Blocks
- [x] Convert BlockEntities
- [ ] Convert Entities
- [ ] Convert Block "palettes"
- [x] Web Interface
- [ ] CLI Interface
- [ ] API

The tool is still able to convert most structures. However, there are still some issues with the conversion of certain blocks and block entities. If you encounter any issues, please report them in the issues tab.

---

#### How to use:

1. Head over to the [web interface](http://converter.chaoscaot.fun/)
2. Upload your structure file
3. Done!

---

#### Why this project?

The Minecraft Java space is dominated by Schematics, while the Bedrock space is dominated by Structures. This project aims to bridge the gap between the two formats, allowing for easier sharing of builds between the two versions.

---

### Building the project

#### Requirements:
- Node.js
- pnpm
- Rust/Cargo
- wasm-pack

#### Steps:

1. Clone the repository
2. Run `wasm-pack build` in the s2s_web directory
3. Run `pnpm install`
4. Run `pnpm build`
5. Run `pnpm dev` to start the development server