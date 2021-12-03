Module Program
    Sub Main(args As String())
        Const sampleInput = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"

        Console.WriteLine(GammaRate2(sampleInput))
        Console.ReadKey()
    End Sub

    Public Function GammaRate1(input As String) As Int32
        Dim i = 0
        Dim split = input.Replace(Chr(13), "") _
            .Split(Chr(10)) _
            .Select(Function(r) r.ToCharArray()) _
            .ToArray()

        Dim gamma = ""
        Dim epsilon = ""
        Dim maxLen = split.First().Length

        While True
            Dim acc = New With {
                .Zeros = 0,
                .Ones = 0
            }

            For Each row In split
                If row(i) = "0"c Then
                    acc.Zeros += 1
                Else
                    acc.Ones += 1
                End If
            Next

            i += 1

            If acc.Zeros > acc.Ones Then
                gamma += "0"c
                epsilon += "1"c
            Else
                epsilon += "0"c
                gamma += "1"c
            End If

            If i >= maxLen Then
                Exit While
            End If
        End While

        Return Convert.ToInt32(gamma, 2) * Convert.ToInt32(epsilon, 2)
    End Function

    Public Function GammaRate2(input As String) As Int32
        Dim allBits = input.Replace(Chr(13), "") _
            .Split(Chr(10)) _
            .Select(Function(r) r.ToCharArray()) _
            .ToArray()

        Dim o2 = FilterFor2(FilterMode.MostCommon, allBits)
        Dim co2 = FilterFor2(FilterMode.LeastCommon, allBits)

        Return Convert.ToInt32(o2, 2) * Convert.ToInt32(co2, 2)
    End Function

    Enum FilterMode
        MostCommon
        LeastCommon
    End Enum

    Public Function FilterFor2(mode As FilterMode, input As Char()()) As String
        Dim i = 0

        While True
            Dim acc = New With {
                .Zeros = 0,
                .Ones = 0
            }

            For Each row In input
                If row(i) = "0"c Then
                    acc.Zeros += 1
                Else
                    acc.Ones += 1
                End If
            Next

            If acc.Zeros = acc.Ones Then
                acc.Ones += 1
            End If

            Select Case mode
                Case FilterMode.LeastCommon
                    input = input.Where(Function(r) If(acc.Zeros < acc.Ones, r(i) = "0"c, r(i) = "1"c)).ToArray()
                Case FilterMode.MostCommon
                    input = input.Where(Function(r) If(acc.Zeros > acc.Ones, r(i) = "0"c, r(i) = "1"c)).ToArray()
                Case Else
                    Throw New Exception("Oh no")
            End Select

            If input.Length = 1 Then
                Exit While
            End If

            i += 1
        End While

        Return input.First()
    End Function
End Module
