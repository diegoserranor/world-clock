# world clock
App that lets you manage your own list of clocks for cities around the world
- Core services written in Rust.
- Tauri state management and commands to retrieve and save data.
- Clock data persisted in simple JSON file.
- Frontend built with React written in TypeScript.
- Styled with Tailwind CSS.
- Integrated with a list of cities from the GeoNames database.
- Finite State Transducer for fast prefix lookups for the list of cities.

## Develop
Run the program from the root of the repo with `pnpm tauri dev`. This will run both the backend and the frontend in development mode, and will restart when it detects changes.

