Imports System.Runtime.CompilerServices

Module Program
    Public Structure Point
        Public x As Int32
        Public y As Int32
    End Structure

    <Extension>
    Public Function IsAdjecent(lhs As Point, rhs As Point) As Boolean
        Return lhs.x >= rhs.x - 1 And lhs.x <= rhs.x + 1 And lhs.y >= rhs.y - 1 And lhs.y <= rhs.y + 1
    End Function

    Public Function CreatePoint(x As Int32, y As Int32) As Point
        Return New Point With {.x = x, .y = y}
    End Function

    Public Class LowPoint
        Private ReadOnly _p As Point
        Private ReadOnly _h As Int32

        Public Sub New(x As Int32, y As Int32, h As Int32)
            _p = CreatePoint(x, y)
            _h = h
        End Sub

        Public Function IsAdjacent(x As Int32, y As Int32) As Boolean
            Return _p.IsAdjecent(CreatePoint(x, y))
        End Function

        Public Function RiskLevel() As Int32
            Return _h + 1
        End Function
    End Class

    Public Class Heightmap
        Private ReadOnly _map As Char()()

        Private ReadOnly _lowPoints As List(Of LowPoint)

        Private ReadOnly _highPoints As List(Of Point)

        Private ReadOnly _basins As List(Of List(Of Point))

        Private ReadOnly _xMax As Int32

        Private ReadOnly _yMax As Int32

        Public Sub New(input As String)
            _map = input.Split(New String() {Environment.NewLine}, StringSplitOptions.None) _
                .Select(Function(r) r.ToCharArray) _
                .ToArray()

            _lowPoints = New List(Of LowPoint)
            _highPoints = New List(Of Point)
            _basins = New List(Of List(Of Point))
            _xMax = _map.Length
            _yMax = _map.First().Length
        End Sub

        Public Function RiskSum() As Int32
            Return _lowPoints.Aggregate(0, Function(a, r) a + r.RiskLevel())
        End Function

        Public Sub CrunchNumbers()
            Dim levels = New List(Of Char) From {"0"c, "1"c, "2"c, "3"c, "4"c, "5"c, "6"c, "7"c, "8"c, "9c"}

            For Each l In levels
                FilterLevel(l)
            Next
        End Sub

        Public Sub CrunchBasins()
            Dim x = 0
            For Each row In _map
                Dim y = 0
                For Each col In row
                    If Not col.Equals("9"c) And Not IsPartOfBasin(x, y) Then
                        _basins.Add(CalculateBasin(x, y, New List(Of Point) From {CreatePoint(x, y)}))
                    End If
                    y += 1
                Next
                x += 1
            Next
        End Sub

        Public Function BasinsRiskResult() As Int32
            _basins.Sort(Function(a, b) b.Count.CompareTo(a.Count))
            Dim top3 = _basins.Take(3)
            Return top3(0).Count * top3(1).Count * top3(2).Count
        End Function

        Private Sub FilterLevel(level As Char)
            Dim x = 0
            For Each row In _map
                Dim y = 0
                For Each col In row
                    If level = col Then
                        If AdjacentToSomePoint(x, y) Then
                            _highPoints.Add(New Point With {.x = x, .y = y})
                        Else
                            _lowPoints.Add(New LowPoint(x, y, Int32.Parse(col)))
                        End If
                    End If
                    y += 1
                Next
                x += 1
            Next
        End Sub

        Private Function AdjacentToSomePoint(x As Int32, y As Int32) As Boolean
            For Each point In _lowPoints
                If point.IsAdjacent(x, y) Then
                    Return True
                End If
            Next

            For Each point In _highPoints
                If point.IsAdjecent(CreatePoint(x, y)) Then
                    Return True
                End If
            Next

            Return False
        End Function

        Private Function IsPartOfBasin(x As Int32, y As Int32) As Boolean
            For Each basin In _basins
                For Each point In basin
                    If point.x = x And point.y = y Then
                        Return True
                    End If
                Next
            Next
            Return False
        End Function

        Private Function PointExistsOnMap(p As Point) As Boolean
            Return p.x >= 0 And p.x < _xMax And p.y >= 0 And p.y < _yMax
        End Function

        Private Function CalculateBasin(x As Int32, y As Int32, acc As List(Of Point)) As List(Of Point)
            Dim points() = New Point() {
                CreatePoint(x - 1, y),
                CreatePoint(x + 1, y),
                CreatePoint(x, y - 1),
                CreatePoint(x, y + 1)
            }

            For Each p In points
                If PointExistsOnMap(p) And Not acc.Contains(p) Then
                    If Not _map(p.x)(p.y).Equals("9"c) Then
                        acc.Add(p)
                        acc = CalculateBasin(p.x, p.y, acc)
                    End If
                End If
            Next

            Return acc
        End Function
    End Class

    Sub Main(args As String())
        Dim map = New Heightmap(Input.SampleInput)

        ' Part 1
        map.CrunchNumbers()
        Console.WriteLine(map.RiskSum())

        ' Part 2
        map.CrunchBasins()
        Console.WriteLine(map.BasinsRiskResult())
        Console.ReadKey()
    End Sub
End Module
