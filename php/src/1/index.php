<?php

$numbers = [
    'zero' => 0,
    'one' => 1,
    'two' => 2,
    'three' => 3,
    'four' => 4,
    'five' => 5,
    'six' => 6,
    'seven' => 7,
    'eight' => 8,
    'nine' => 9,
];

/**
 * @return integer
 */
function getFirst(string $line): int
{
    $lineLenth = strlen($line);

    for ($i = 0; $i < $lineLenth; $i++) {
        if (is_numeric($line[$i])) {
            return (int) $line[$i];
        }

        foreach ($GLOBALS['numbers'] as $number => $value) {
            if (numberIsPart($line, $i, $number)) {
                return $value;
            }
        }
    }

    throw new Exception('No number found');
}

/**
 * @return integer
 */
function getLast(string $line): int
{
    for ($i = strlen($line) - 1; $i >= 0; $i--) {
        if (is_numeric($line[$i])) {
            return (int) $line[$i];
        }

        foreach ($GLOBALS['numbers'] as $number => $value) {
            if (numberIsPart($line, $i - strlen($number), $number)) {
                return $value;
            }
        }
    }

    throw new Exception('No number found');
}

/**
 * @return integer
 */
function getLineValue(string $line): int
{
    return (getFirst($line) * 10) + getLast($line);
}

/**
 * @return boolean
 */
function numberIsPart(string $line, int $start, string $number): bool
{
    return substr($line, $start, strlen($number)) === $number;
}

$filename = $argv[1];

$file = fopen($filename, 'r');
$sum = 0;

while ($line = fgets($file)) {
    $sum += getLineValue($line);
}

fclose($file);

echo $sum . PHP_EOL;