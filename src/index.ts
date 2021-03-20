#!/usr/bin/env node

/*
 This program allows you to work with PNG image format,
 you can write a secret message and put it in the image file
 containing any of the ASCII codes or Emoji.

 The program supports loading images via the URI or from the local computer.
 Have the option of writing a message into the image by pattern –
 distance between the bits of the message.

 Also the program can get the message
 from already encoded PNG image.

 A program developed by Andrey Medvedev in 2016.
 The program code is completely open for your modifications.

 TO WORK WITH THE PROGRAM YOU NEED TO INSTALL NODEJS.
 THE EASIEST WAY TO INSTALL SOFTWARE USING NODE PACKAGE MANAGER (npm i -g stegjs).
 */


import { program } from 'commander';
import { green, red, yellow, cyan } from 'chalk';
import { join } from 'path';


/**
 * Global variables
 * @type {null}
 */
let imageValue;
let messageValue;
let stepValue;
let outputValue;


/** It describes the options and arguments that the program accepts. */
program
    .version(require('../package').version)
    .description('Encrypt you message to PNG image.')
    .arguments('<image> [message] [step] [output]')
    .usage('<image or url>.png [mode] [message] [step] [output]')
    .option('-e, --encode', 'Change program mode to encode file.')
    .option('-d, --decode', 'Change program mode to decode file');


/** This is block examples of working with the program. */
program.on('--help', () => {
    console.log('  Examples:');
    console.log('');
    console.log(`    $ ${green('stegjs')} ${yellow('photos/IMG.png')} --encode ${cyan("'Secret message'")} 10x10 ${yellow('secrets/IMG.png')}`);
    console.log(`    $ ${green('stegjs')} ${yellow('http://google.com/img.png')} --encode ${cyan("'Secret message'")} 1x1 ${yellow('secrets/IMG.png')}`);
    console.log(`    $ ${green('stegjs')} ${yellow('secrets/IMG.png')} --decode`);
    console.log('');
});


/** The arguments passed by the program. */
program.action((image, message, step, output) => {
    imageValue = image;
    messageValue = message;
    stepValue = step;
    outputValue = output;
});


/** Parse arguments that come into the program. */
program.parse(process.argv);


/** If there is no input image, then throw an error to the console. */
if (typeof imageValue === 'undefined') {
    console.error(red('No image given! Please see help with next command: $ stegjs --help'));
    process.exit(1);
}


/** If user does not specify output path, then throw output image to the current folder with name out.png */
outputValue = outputValue || join(process.cwd(), '/out.png');


/** Branching according of the program mode. */
if (program.opts()["encode"]) require('./encode.js').encode(imageValue, messageValue, stepValue, outputValue);
if (program.opts()["decode"]) require('./decode.js').decode(imageValue);

export * from "./encode";
export * from "./decode";
