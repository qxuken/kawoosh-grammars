program Sample;

{ A sample. }

uses
  SysUtils;

const
  Limit = 10;

type
  TPoint = record
    X, Y: Double;
  end;

  TShape = class
  private
    FRadius: Double;
  public
    constructor Create(ARadius: Double);
    function Area: Double;
  end;

constructor TShape.Create(ARadius: Double);
begin
  FRadius := ARadius;
end;

function TShape.Area: Double;
begin
  Result := Pi * FRadius * FRadius;
end;

function Largest(const Items: array of Integer): Integer;
var
  I: Integer;
begin
  Result := Items[0];
  for I := 1 to High(Items) do
    if Items[I] > Result then
      Result := Items[I];
end;

var
  Shape: TShape;
  P: TPoint;
begin
  P.X := 3.0;
  P.Y := 4.0;
  Shape := TShape.Create(1.0);
  try
    // A comment.
    if Shape.Area < Limit then
      WriteLn('small: ', Shape.Area:0:2)
    else
      WriteLn('large');
    WriteLn(Largest([1, 2, 3]), ' ', Sqrt(P.X * P.X + P.Y * P.Y):0:1);
  finally
    Shape.Free;
  end;
end.
