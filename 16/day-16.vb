Imports System.Runtime.CompilerServices
Imports System.Text

Module ArrayExt
    <Extension>
    Public Function Slice(Of T)(lhs As T(), from As Int32, length As Int32) As T()
        Dim result = New T(length) {}
        Array.Copy(lhs, from, result, 0, length)
        Return result
    End Function
End Module

Module Program
    Class Packet
        Public Sub New(literal As String)
            Dim bytes = literal _
                .Select(Function(r) Convert.ToString(Convert.ToInt32(r, 16), 2).PadLeft(4, "0")) _
                .ToArray()
            Me.Input = Join(bytes, "")

            Me.SubPackets = New List(Of Packet)
            Me.EndPosition = 0
        End Sub

        Sub New(literal As String, raw As Boolean)
            Input = literal
            Me.SubPackets = New List(Of Packet)
            Me.EndPosition = 0
        End Sub

        Private ReadOnly Property Input As String

        Public Property EndPosition As UInt64

        Public Property VersionNumber As UInt64

        Public Property SubPackets As List(Of Packet)

        Public Function Parse() As UInt64
            VersionNumber = Convert.ToUInt64(Input.Substring(0, 3), 2)
            Dim typeId = Convert.ToUInt64(Input.Substring(3, 3), 2)

            If typeId = 4 Then
                Dim getStartingIndex = Function(e As UInt64) 6 + e * 5
                Dim i = 0
                Dim acc = ""
                While True
                    Dim entry = Input.Substring(getStartingIndex(i), 5)
                    acc += entry.Substring(1, 4)
                    i += 1
                    If entry(0) = "0"c Then
                        Exit While
                    End If
                End While
                EndPosition = getStartingIndex(i)
                Return Convert.ToUInt64(acc, 2)
            End If

            Dim lenTypeId = Input.Substring(6, 1)
            Dim subStart As UInt64
            Dim parseResults = New List(Of UInt64)

            If lenTypeId = "0"c Then
                Dim len = Convert.ToUInt64(Input.Substring(7, 15), 2)
                subStart = 22

                While True
                    Dim innerPacket = New Packet(Input.Substring(subStart, Input.Length - subStart), True)
                    parseResults.Add(innerPacket.Parse())
                    SubPackets.Add(innerPacket)
                    subStart += innerPacket.EndPosition

                    If subStart - 22 = len Then
                        Exit While
                    End If
                End While
            Else
                Dim subCount = Convert.ToUInt64(Input.Substring(7, 11), 2)
                subStart = 18
                Dim i = 0

                While True
                    Dim innerPacket = New Packet(Input.Substring(subStart, Input.Length - subStart), True)
                    parseResults.Add(innerPacket.Parse())
                    SubPackets.Add(innerPacket)
                    subStart += innerPacket.EndPosition

                    i += 1
                    If i >= subCount Then
                        Exit While
                    End If
                End While
            End If

            EndPosition += subStart

            Select Case typeId
                Case 0 ' Sum
                    Return parseResults.Aggregate(UInt64.Parse("0"), Function(a, r) a + r)
                Case 1 ' Product
                    Return parseResults.Aggregate(UInt64.Parse("1"), Function(a, r) a * r)
                Case 2 ' Min
                    Return parseResults.Min()
                Case 3 ' Max
                    Return parseResults.Max()
                Case 5 ' Greater then
                    Return If(parseResults(0) > parseResults(1), 1, 0)
                Case 6 ' Less then
                    Return If(parseResults(0) < parseResults(1), 1, 0)
                Case 7 ' Equal
                    Return If(parseResults(0) = parseResults(1), 1, 0)
                Case Else
                    Throw New Exception("Unreachable")
            End Select
        End Function

        Public Function SubPacketSum() As UInt64
            Return SubPackets.Aggregate(VersionNumber, Function(a, r) a + r.SubPacketSum())
        End Function
    End Class

    Sub Main(args As String())
        Dim packet = New Packet(bigInput)
        Dim parsed = packet.Parse()

        Console.WriteLine($"Part 1: {packet.SubPacketSum}")
        Console.WriteLine($"Part 2: {parsed}")
        Console.ReadKey()
    End Sub
End Module
