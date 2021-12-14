Imports System.IO
Imports System.Runtime.CompilerServices

Module IteratableExt
    <Extension>
    Public Function Chunks(Of T)(lhs As IEnumerable(Of T), chunkSize As Int32) As IEnumerable(Of IEnumerable(Of T))
        If chunkSize = 0 Then
            Throw New Exception("Invalid chunk size")
        End If

        Dim acc = New List(Of List(Of T))
        Dim innerAcc = New List(Of T)

        For Each row In lhs
            innerAcc.Add(row)
            If innerAcc.Count = chunkSize Then
                acc.Add(innerAcc)
                innerAcc = New List(Of T)
            End If
        Next

        Return acc
    End Function
End Module

Module Program
    Class Polymerization
        Private Property Instructions As Dictionary(Of String, Char)

        Private Property Acc As New Dictionary(Of String, Long)

        Private Property PolyInput As String

        Public Sub New(input As String)
            Instructions = New Dictionary(Of String, Char)
            Dim split = input.Split(New String() {Environment.NewLine, " "c}, StringSplitOptions.RemoveEmptyEntries) _
                    .ToList()

            Acc = New Dictionary(Of String, Long)
            Dim reader = New StringReader(split(0))
            PolyInput = split(0)
            split.RemoveAt(0)

            While True
                Dim lhs = ChrW(reader.Read())
                If reader.Peek() = -1 Then
                    Exit While
                End If

                Dim rhs = ChrW(reader.Peek())
                Acc(lhs + rhs) = Acc.GetValueOrDefault(lhs + rhs) + 1
            End While

            For Each row In split.Chunks(3)
                Instructions.Add(row(0), row(2))
            Next
        End Sub

        Public Sub FireRound()
            Dim newAcc = New Dictionary(Of String, Long)

            For Each pair In Acc
                Dim lhs = pair.Key(0)
                Dim rhs = pair.Key(1)
                Dim middle = Instructions(lhs + rhs)

                newAcc(lhs + middle) = newAcc.GetValueOrDefault(lhs + middle) + pair.Value
                newAcc(middle + rhs) = newAcc.GetValueOrDefault(middle + rhs) + pair.Value
            Next

            Acc = newAcc
        End Sub

        Public Function Subtraction() As Long
            Dim counts = New Dictionary(Of Char, Long)

            For Each pair In Acc
                counts(pair.Key(0)) = counts.GetValueOrDefault(pair.Key(0)) + pair.Value
                counts(pair.Key(1)) = counts.GetValueOrDefault(pair.Key(1)) + pair.Value
            Next

            counts(PolyInput.Last) += 1

            Return (counts.Values.Max - counts.Values.Min) / 2
        End Function
    End Class

    Sub Main(args As String())
        Dim poly = New Polymerization(bigInput)

        ' Part 1
        Dim i = 0
        While i < 10
            poly.FireRound()
            i += 1
        End While
        Console.WriteLine(poly.Subtraction)

        ' Part 2
        While i < 40
            poly.FireRound()
            i += 1
        End While
        Console.WriteLine(poly.Subtraction)
        Console.ReadKey()
    End Sub
End Module
