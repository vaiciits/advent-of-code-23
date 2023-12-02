<?php

require_once __DIR__ . '\..\common\Puzzle.php';

class Puzzle2 extends Puzzle
{
    /**
     * Sum of valid game ids.
     *
     * @var integer
     */
    public int $answer = 0;

    /**
     * Sum of min set size multiplication.
     *
     * @var integer
     */
    public int $answer2 = 0;

    /**
     * @var <string, int>
     */
    public array $colors = [
        'red' => 12,
        'green' => 13,
        'blue' => 14,
    ];

    /**
     * @return integer
     */
    public function getLineValue(string $line): int
    {
        $data = explode(':', $line);

        foreach (explode(';', $data[1]) as $set) {
            if (!$this->validSet($set)) {
                return 0;
            }
        }

        return (int) explode(' ', $data[0])[1];
    }

    /**
     * @return integer
     */
    public function getMinSetSize(string $line): int
    {
        $data = explode(':', $line);
        $sets = explode(';', $data[1]);
        $colors = [
            'red' => 0,
            'green' => 0,
            'blue' => 0,
        ];

        foreach ($sets as $set) {
            $cubes = explode(',', $set);

            foreach ($cubes as $cube) {
                $cubeData = explode(' ', trim($cube));

                if ($colors[$cubeData[1]] < (int) $cubeData[0]) {
                    $colors[$cubeData[1]] = (int) $cubeData[0];
                }
            }
        }

        return $colors['red'] * $colors['green'] * $colors['blue'];
    }

    /**
     * @return boolean
     */
    public function validSet(string $set): bool
    {
        $cubes = explode(',', $set);

        foreach ($cubes as $cube) {
            $data = explode(' ', trim($cube));

            if ($this->colors[$data[1]] < (int) $data[0]) {
                return false;
            }
        }

        return true;
    }

    /**
     * {@inheritDoc}
     */
    public function solve(): static
    {
        $lines = $this->inputAsArray();

        foreach ($lines as $line) {
            $this->answer += $this->getLineValue($line);
            $this->answer2 += $this->getMinSetSize($line);
        }

        return $this;
    }
}

$puzzle = new Puzzle2($argv[1]);
$puzzle->run();
echo 'Part 1: ' . $puzzle->answer . PHP_EOL;
echo 'Part 2: ' . $puzzle->answer2 . PHP_EOL;