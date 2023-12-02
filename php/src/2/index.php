<?php

require_once __DIR__ . '\..\common\Puzzle.php';

class Puzzle2 extends Puzzle
{
    /**
     * @var integer
     */
    public int $answer = 0;

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
        }

        return $this;
    }
}

$puzzle = new Puzzle2($argv[1]);
echo $puzzle->run()->answer . PHP_EOL;