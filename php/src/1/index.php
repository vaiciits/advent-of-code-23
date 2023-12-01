<?php

/**
 * @param string $line
 * @return integer
 */
function getFirst(string $line): int
{
    foreach (str_split($line) as $char) {
        if (is_numeric($char)) {
            return (int) $char;
        }
    }

    throw new Exception('No number found');
}

/**
 * @param string $line
 * @return integer
 */
function getLast(string $line): int
{
    for ($i = strlen($line) - 1; $i >= 0; $i--) {
        if (is_numeric($line[$i])) {
            return (int) $line[$i];
        }
    }

    throw new Exception('No number found');
}

/**
 * @param string $line
 * @return integer
 */
function getLineValue(string $line): int
{
    return (getFirst($line) * 10) + getLast($line);
}

$filename = $argv[1];

$file = fopen($filename, 'r');
$sum = 0;

while ($line = fgets($file)) {
    $sum += getLineValue($line);
}

fclose($file);

echo $sum . PHP_EOL;