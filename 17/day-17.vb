Imports System.Text.RegularExpressions

Module Program
    Class ProbeLauncher
        Public Sub New(input As String)
            Dim groups = Regex.Matches(input, "(-?\d+\.\.-?\d+)") _
                .Select(Function(r) r.Value.Split("..")) _
                .ToList()

            XMin = groups(0)(0)
            XMax = groups(0)(1)

            YMin = groups(1)(0)
            YMax = groups(1)(1)
        End Sub

        Private Property XMin As Int32

        Private Property XMax As Int32

        Private Property YMin As Int32

        Private Property YMax As Int32

        Public Function FireProbe(xVelosity As Int32, yVelosity As Int32) As Boolean
            Dim xPosition = 0
            Dim yPosition = 0

            While True
                ' SATs
                If xPosition > XMax Then ' Shot past
                    Return False
                End If
                If xPosition < XMin And xVelosity = 0 Then ' Shot not far enough
                    Return False
                End If
                If yPosition < YMin Then ' Shot under
                    Return False
                End If
                If xPosition >= XMin And xPosition <= XMax And yPosition >= YMin And yPosition <= YMax Then ' Hit target
                    Return True
                End If

                xPosition += xVelosity
                yPosition += yVelosity

                ' Update velosities
                If xVelosity < 0 Then
                    xVelosity += 1
                ElseIf xVelosity > 0 Then
                    xVelosity -= 1
                End If
                yVelosity -= 1
            End While
        End Function

        Private Shared Function MaxHeightForShot(yVelosity As Int32)
            Dim yPosition = 0

            While True
                If yVelosity <= 0 Then
                    Return yPosition
                End If
                yPosition += yVelosity
                yVelosity -= 1
            End While
        End Function

        Public Function MaxHeight() As Int32
            Dim i = 0
            Dim x = 0

            ' Get a x position that will always hit
            While True
                If i >= XMin And i <= XMax Then
                    Exit While
                End If
                If XMin > 0 Then
                    x += 1
                    i += x
                ElseIf XMin < 0 Then
                    x -= 1
                    i -= 1
                End If
            End While

            Dim best = 0

            For Each y In Enumerable.Range(0, 0 - YMin)
                Dim max = MaxHeightForShot(y)
                If FireProbe(x, y) And max > best Then
                    best = MaxHeightForShot(y)
                End If
            Next

            Return best
        End Function

        Public Function AllHitsCount() As Int32
            Dim i = 0

            For Each x In Enumerable.Range(0, XMax * 2)
                For Each y In Enumerable.Range(YMin, Math.Abs(YMin) * Math.Abs(YMax))
                    i += If(FireProbe(x, y), 1, 0)
                Next
            Next

            Return i
        End Function
    End Class

    Sub Main(args As String())
        Dim launcher = New ProbeLauncher(GameInput)
        Dim result = launcher.MaxHeight()

        Console.WriteLine($"Part 1: {launcher.MaxHeight()}")
        Console.WriteLine($"Part 2: {launcher.AllHitsCount()}")
        Console.ReadKey()
    End Sub
End Module
