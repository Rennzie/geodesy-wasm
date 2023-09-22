import {Command, Option} from 'commander';

const program = new Command();

const args = program
  .addOption(
    new Option('-n, --name <type>', 'Name of the example to run')
      .default('00-basics')
      .choices(['00-basics', '01-gridshift']),
  )
  .parse()
  .opts();

const proc = Bun.spawn(['bun', 'run', `${import.meta.dir}/${args.name}.ts`], {
  stdout: 'inherit',
});
