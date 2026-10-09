interface Point {
  x: number;
  y: number;
}

interface SetPoint {
  (x: number, y: number): void;
}

type Point = {
  x: number;
  y: number;
};

type SetPoint = (x: number, y: number) => void;

// primitive
type Name = string;

// object
type PartialPointX = { x: number; };
type PartialPointY = { y: number; };

// union
type PartialPoint = PartialPointX | PartialPointY;

// tuple
type Data = [number, string];

interface PartialPointX { x: number; }
interface Point extends PartialPointX { y: number; }

type PartialPointX = { x: number; };
type Point = PartialPointX & { y: number; };

type PartialPointX = { x: number; };
interface Point extends PartialPointX { y: number; }

interface PartialPointX { x: number; }
type Point = PartialPointX & { y: number; };

interface Point {
  x: number;
  y: number;
}

class SomePoint implements Point {
  x = 1;
  y = 2;
}

type Point2 = {
  x: number;
  y: number;
};

class SomePoint2 implements Point2 {
  x = 1;
  y = 2;
}

type PartialPoint = { x: number; } | { y: number; };

// FIXME: can not implement a union type
class SomePartialPoint implements PartialPoint {
  x = 1;
  y = 2;
}

// These two declarations become:
// interface Point { x: number; y: number; }
interface Point { x: number; }
interface Point { y: number; }

const point: Point = { x: 1, y: 2 };