<?php

abstract class Puzzle
{
    /**
     * @var string
     */
    public string $input = '';

    public function __construct(protected string $filename)
    {
    }

    /**
     * @return string[]
     */
    public function inputAsArray(): array
    {
        return explode(PHP_EOL, $this->input);
    }

    /**
     * @return static
     */
    public function loadInput(): static
    {
        $this->input = file_get_contents($this->filename);

        return $this;
    }

    /**
     * Prepares the output.
     *
     * @return static
     */
    public function run(): static
    {
        return $this->loadInput()->solve();
    }

    /**
     * @return static
     */
    abstract public function solve(): static;
}