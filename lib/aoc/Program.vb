Imports System

Module Program
    Sub Main(args As String())
        Dim submarine = New Submarine2
        Dim sampleInput = "forward 5
down 5
forward 8
up 3
down 8
forward 2"

        For Each cmd In sampleInput.Replace(Chr(13), "").Split(Chr(10))
            submarine.FireCommand(cmd)
        Next

        Console.WriteLine(submarine.Result())
        Console.ReadKey()
    End Sub


    Class Submarine1
        Private _posForward = 0
        Private _posDepth = 0

        Public Sub FireCommand(command As String)
            Dim split = command.Split(" ")
            Dim name = split(0)
            Dim position = Int32.Parse(split(1))

            Select Case name
                Case "down"
                    _posDepth += position
                Case "forward"
                    _posForward += position
                Case "up"
                    _posDepth -= position
            End Select
        End Sub

        Public Function Result() As Int32
            Return _posForward * _posDepth
        End Function
    End Class
    Class Submarine2
        Private _posForward = 0
        Private _posAim = 0
        Private _posDepth = 0

        Public Sub FireCommand(command As String)
            Dim split = command.Split(" ")
            Dim name = split(0)
            Dim position = Int32.Parse(split(1))

            Select Case name
                Case "down"
                    _posAim += position
                Case "up"
                    _posAim -= position
                Case "forward"
                    _posForward += position
                    _posDepth += position * _posAim
            End Select
        End Sub

        Public Function Result() As Int32
            Return _posForward * _posDepth
        End Function
    End Class
End Module
