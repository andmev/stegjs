import child from 'child_process';
import { join } from 'path';
import { promisify } from 'util';

const exec = promisify(child.exec);

describe('Main Functionality', () => {
  it('prints StegJS version', async () => {
    const { stdout, stderr } = await exec(
      `ts-node ${join(__dirname, 'index.ts')} --version`,
    );
    expect(stderr).toEqual('');
    expect(stdout).toEqual('2.0.6\n');
  });
  it('prints StegJS help information', async () => {
    const { stdout, stderr } = await exec(
      `ts-node ${join(__dirname, 'index.ts')} --help`,
    );
    expect(stderr).toEqual('');
    expect(stdout).toContain('Usage:');
    expect(stdout).toContain('Encrypt you message to PNG image.');
    expect(stdout).toContain('Options:');
    expect(stdout).toContain('Examples:');
  });
});
