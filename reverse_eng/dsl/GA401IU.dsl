/*
 * Intel ACPI Component Architecture
 * AML/ASL+ Disassembler version 20200717 (64-bit version)
 * Copyright (c) 2000 - 2020 Intel Corporation
 * 
 * Disassembling to non-symbolic legacy ASL operators
 *
 * Disassembly of /tmp/fwts_tmp_table_43032_DSDT_4.dsl, Sat Aug  1 08:48:56 2020
 *
 * Original Table Header:
 *     Signature        "DSDT"
 *     Length           0x0000B6BD (46781)
 *     Revision         0x02
 *     Checksum         0x44
 *     OEM ID           "ALASKA"
 *     OEM Table ID     "A M I "
 *     OEM Revision     0x01072009 (17244169)
 *     Compiler ID      "INTL"
 *     Compiler Version 0x20120913 (538052883)
 */
DefinitionBlock ("", "DSDT", 2, "ALASKA", "A M I ", 0x01072009)
{
    External (_SB_.ALIB, MethodObj)    // 2 Arguments
    External (_SB_.APTS, MethodObj)    // 1 Arguments
    External (_SB_.AWAK, MethodObj)    // 1 Arguments
    External (_SB_.PCI0.GP17.VGA_.AFN7, MethodObj)    // 1 Arguments
    External (_SB_.PCI0.GPP0.PEGP, UnknownObj)
    External (_SB_.UBTC, UnknownObj)
    External (_SB_.UBTC.CCI0, FieldUnitObj)
    External (_SB_.UBTC.CCI1, FieldUnitObj)
    External (_SB_.UBTC.CCI2, FieldUnitObj)
    External (_SB_.UBTC.CCI3, FieldUnitObj)
    External (_SB_.UBTC.MGI0, FieldUnitObj)
    External (_SB_.UBTC.MGI1, FieldUnitObj)
    External (_SB_.UBTC.MGI2, FieldUnitObj)
    External (_SB_.UBTC.MGI3, FieldUnitObj)
    External (_SB_.UBTC.MGI4, FieldUnitObj)
    External (_SB_.UBTC.MGI5, FieldUnitObj)
    External (_SB_.UBTC.MGI6, FieldUnitObj)
    External (_SB_.UBTC.MGI7, FieldUnitObj)
    External (_SB_.UBTC.MGI8, FieldUnitObj)
    External (_SB_.UBTC.MGI9, FieldUnitObj)
    External (_SB_.UBTC.MGIA, FieldUnitObj)
    External (_SB_.UBTC.MGIB, FieldUnitObj)
    External (_SB_.UBTC.MGIC, FieldUnitObj)
    External (_SB_.UBTC.MGID, FieldUnitObj)
    External (_SB_.UBTC.MGIE, FieldUnitObj)
    External (_SB_.UBTC.MGIF, FieldUnitObj)
    External (AFN4, MethodObj)    // 1 Arguments
    External (M009, MethodObj)    // 1 Arguments
    External (M010, MethodObj)    // 2 Arguments
    External (M900, MethodObj)    // 1 Arguments

    Name (PEBL, 0x08000000)
    Name (NBTS, 0x5000)
    Name (CPVD, One)
    Name (SMBB, 0x0B20)
    Name (SMBL, 0x20)
    Name (SMB0, 0x0B00)
    Name (SMBM, 0x10)
    Name (PMBS, 0x0800)
    Name (PMLN, 0xA0)
    Name (SMIO, 0xB2)
    Name (APCB, 0xFEC00000)
    Name (APCL, 0x1000)
    Name (HPTB, 0xFED00000)
    Name (WDTB, Zero)
    Name (WDTL, Zero)
    Name (GIOB, 0xFED81500)
    Name (IOMB, 0xFED80D00)
    Name (SSMB, 0xFED80200)
    Name (CAFS, 0xBB)
    Name (UTDB, Zero)
    Name (ASSB, Zero)
    Name (AOTB, Zero)
    Name (AAXB, Zero)
    Name (PEHP, One)
    Name (SHPC, Zero)
    Name (PEPM, One)
    Name (PEER, One)
    Name (PECS, One)
    Name (ITKE, Zero)
    Name (PEBS, 0xF0000000)
    Name (PELN, 0x08000000)
    Name (CSMI, 0x61)
    Name (SMIA, 0xB2)
    Name (SMIB, 0xB3)
    Name (OFST, 0x35)
    Name (TRST, 0x02)
    Name (TCMF, Zero)
    Name (TMF1, Zero)
    Name (TMF2, Zero)
    Name (TMF3, Zero)
    Name (TTPF, Zero)
    Name (DTPT, Zero)
    Name (TTDP, One)
    Name (TPMB, 0xAC35F000)
    Name (TPBS, 0x4000)
    Name (TPMC, 0xAC363000)
    Name (TPCS, 0x4000)
    Name (TPMM, 0xFD310510)
    Name (FTPM, 0xFD310510)
    Name (PPIM, 0xAC56FF98)
    Name (PPIL, 0x1C)
    Name (AMDT, One)
    Name (TPMF, One)
    Name (PPIV, One)
    Name (MBEC, Zero)
    Name (NBTP, 0xFEC00000)
    Name (MBLF, 0x0A)
    Method (_PIC, 1, NotSerialized)  // _PIC: Interrupt Model
    {
        If (Arg0)
        {
            \_SB.DSPI ()
            \_SB.PCI0.NAPE ()
        }

        PXXX (Arg0)
    }

    OperationRegion (DEB0, SystemIO, 0x80, 0x04)
    Field (DEB0, DWordAcc, NoLock, Preserve)
    {
        DBG8,   32
    }

    Name (PICM, Zero)
    Method (PXXX, 1, NotSerialized)
    {
        If (Arg0)
        {
            Store (0xAA, DBGX) /* \DBGX */
        }
        Else
        {
            Store (0xAC, DBGX) /* \DBGX */
        }

        Store (Arg0, PICM) /* \PICM */
    }

    Name (OSVR, Ones)
    Method (OSFL, 0, NotSerialized)
    {
        If (LNotEqual (OSVR, Ones))
        {
            Return (OSVR) /* \OSVR */
        }

        If (LEqual (PICM, Zero))
        {
            Store (0xAC, DBGX) /* \DBGX */
        }

        Store (0x03, OSVR) /* \OSVR */
        If (CondRefOf (\_OSI, Local0))
        {
            If (_OSI ("Windows 2001"))
            {
                Store (0x04, OSVR) /* \OSVR */
            }

            If (_OSI ("Windows 2001.1"))
            {
                Store (0x05, OSVR) /* \OSVR */
            }

            If (_OSI ("FreeBSD"))
            {
                Store (0x06, OSVR) /* \OSVR */
            }

            If (_OSI ("HP-UX"))
            {
                Store (0x07, OSVR) /* \OSVR */
            }

            If (_OSI ("OpenVMS"))
            {
                Store (0x08, OSVR) /* \OSVR */
            }

            If (_OSI ("Windows 2001 SP1"))
            {
                Store (0x09, OSVR) /* \OSVR */
            }

            If (_OSI ("Windows 2001 SP2"))
            {
                Store (0x0A, OSVR) /* \OSVR */
            }

            If (_OSI ("Windows 2001 SP3"))
            {
                Store (0x0B, OSVR) /* \OSVR */
            }

            If (_OSI ("Windows 2006"))
            {
                Store (0x0C, OSVR) /* \OSVR */
            }

            If (_OSI ("Windows 2006 SP1"))
            {
                Store (0x0D, OSVR) /* \OSVR */
            }

            If (_OSI ("Windows 2009"))
            {
                Store (0x0E, OSVR) /* \OSVR */
            }

            If (_OSI ("Windows 2012"))
            {
                Store (0x0F, OSVR) /* \OSVR */
            }

            If (_OSI ("Windows 2013"))
            {
                Store (0x10, OSVR) /* \OSVR */
            }
        }
        Else
        {
            If (MCTH (_OS, "Microsoft Windows NT"))
            {
                Store (Zero, OSVR) /* \OSVR */
            }

            If (MCTH (_OS, "Microsoft Windows"))
            {
                Store (One, OSVR) /* \OSVR */
            }

            If (MCTH (_OS, "Microsoft WindowsME: Millennium Edition"))
            {
                Store (0x02, OSVR) /* \OSVR */
            }

            If (MCTH (_OS, "Linux"))
            {
                Store (0x03, OSVR) /* \OSVR */
            }

            If (MCTH (_OS, "FreeBSD"))
            {
                Store (0x06, OSVR) /* \OSVR */
            }

            If (MCTH (_OS, "HP-UX"))
            {
                Store (0x07, OSVR) /* \OSVR */
            }

            If (MCTH (_OS, "OpenVMS"))
            {
                Store (0x08, OSVR) /* \OSVR */
            }
        }

        Return (OSVR) /* \OSVR */
    }

    Method (MCTH, 2, NotSerialized)
    {
        If (LLess (SizeOf (Arg0), SizeOf (Arg1)))
        {
            Return (Zero)
        }

        Add (SizeOf (Arg0), One, Local0)
        Name (BUF0, Buffer (Local0){})
        Name (BUF1, Buffer (Local0){})
        Store (Arg0, BUF0) /* \MCTH.BUF0 */
        Store (Arg1, BUF1) /* \MCTH.BUF1 */
        While (Local0)
        {
            Decrement (Local0)
            If (LNotEqual (DerefOf (Index (BUF0, Local0)), DerefOf (Index (BUF1, Local0
                ))))
            {
                Return (Zero)
            }
        }

        Return (One)
    }

    Name (PRWP, Package (0x02)
    {
        Zero, 
        Zero
    })
    Method (GPRW, 2, NotSerialized)
    {
        Store (Arg0, Index (PRWP, Zero))
        Store (ShiftLeft (SS1, One), Local0)
        Or (Local0, ShiftLeft (SS2, 0x02), Local0)
        Or (Local0, ShiftLeft (SS3, 0x03), Local0)
        Or (Local0, ShiftLeft (SS4, 0x04), Local0)
        If (And (ShiftLeft (One, Arg1), Local0))
        {
            Store (Arg1, Index (PRWP, One))
        }
        Else
        {
            ShiftRight (Local0, One, Local0)
            If (LOr (LEqual (OSFL (), One), LEqual (OSFL (), 0x02)))
            {
                FindSetLeftBit (Local0, Index (PRWP, One))
            }
            Else
            {
                FindSetRightBit (Local0, Index (PRWP, One))
            }
        }

        Return (PRWP) /* \PRWP */
    }

    Name (WAKP, Package (0x02)
    {
        Zero, 
        Zero
    })
    Method (UPWP, 1, NotSerialized)
    {
        If (DerefOf (Index (WAKP, Zero)))
        {
            Store (Zero, Index (WAKP, One))
        }
        Else
        {
            Store (Arg0, Index (WAKP, One))
        }
    }

    OperationRegion (DEB3, SystemIO, 0x80, One)
    Field (DEB3, ByteAcc, NoLock, Preserve)
    {
        DBGX,   8
    }

    OperationRegion (DEB1, SystemIO, 0x90, 0x02)
    Field (DEB1, WordAcc, NoLock, Preserve)
    {
        DBG9,   16
    }

    Name (SS1, Zero)
    Name (SS2, Zero)
    Name (SS3, One)
    Name (SS4, One)
    Name (IOST, 0xFFFF)
    Name (TOPM, 0x00000000)
    Name (ROMS, 0xFFE00000)
    Name (VGAF, One)
    OperationRegion (DEB2, SystemIO, 0x80, 0x04)
    Field (DEB2, DWordAcc, NoLock, Preserve)
    {
        P80H,   32
    }

    Name (OSTY, Ones)
    OperationRegion (ACMS, SystemIO, 0x72, 0x02)
    Field (ACMS, ByteAcc, NoLock, Preserve)
    {
        ACMX,   8, 
        ACMA,   8
    }

    IndexField (ACMX, ACMA, ByteAcc, NoLock, Preserve)
    {
        Offset (0xB9), 
        IMEN,   8
    }

    OperationRegion (PSMI, SystemIO, SMIO, 0x02)
    Field (PSMI, ByteAcc, NoLock, Preserve)
    {
        APMC,   8, 
        APMD,   8
    }

    OperationRegion (PMRG, SystemIO, 0x0CD6, 0x02)
    Field (PMRG, ByteAcc, NoLock, Preserve)
    {
        PMRI,   8, 
        PMRD,   8
    }

    IndexField (PMRI, PMRD, ByteAcc, NoLock, Preserve)
    {
            ,   6, 
        HPEN,   1, 
        Offset (0x60), 
        P1EB,   16, 
        Offset (0xF0), 
            ,   3, 
        RSTU,   1
    }

    OperationRegion (GSMG, SystemMemory, 0xFED81500, 0x03FF)
    Field (GSMG, AnyAcc, NoLock, Preserve)
    {
        Offset (0x5C), 
        Offset (0x5E), 
        GS23,   1, 
            ,   5, 
        GV23,   1, 
        GE23,   1, 
        Offset (0xA0), 
        Offset (0xA2), 
        GS40,   1, 
            ,   5, 
        GV40,   1, 
        GE40,   1
    }

    OperationRegion (GSMM, SystemMemory, 0xFED80000, 0x1000)
    Field (GSMM, AnyAcc, NoLock, Preserve)
    {
        Offset (0x288), 
            ,   1, 
        CLPS,   1, 
        Offset (0x296), 
            ,   7, 
        TMSE,   1, 
        Offset (0x2B0), 
            ,   2, 
        SLPS,   2, 
        Offset (0x3BB), 
            ,   6, 
        PWDE,   1
    }

    OperationRegion (P1E0, SystemIO, P1EB, 0x04)
    Field (P1E0, ByteAcc, NoLock, Preserve)
    {
        Offset (0x01), 
            ,   6, 
        PEWS,   1, 
        WSTA,   1, 
        Offset (0x03), 
            ,   6, 
        PEWD,   1
    }

    OperationRegion (IOCC, SystemIO, PMBS, 0x80)
    Field (IOCC, ByteAcc, NoLock, Preserve)
    {
        Offset (0x01), 
            ,   2, 
        RTCS,   1
    }

    Method (SPTS, 1, NotSerialized)
    {
        Store (Arg0, P80H) /* \P80H */
        If (LEqual (Arg0, 0x03))
        {
            Store (Zero, RSTU) /* \RSTU */
        }

        Store (One, CLPS) /* \CLPS */
        Store (One, SLPS) /* \SLPS */
        Store (PEWS, PEWS) /* \PEWS */
        If (LEqual (Arg0, 0x03))
        {
            Store (One, SLPS) /* \SLPS */
        }

        If (LEqual (Arg0, 0x04))
        {
            Store (One, SLPS) /* \SLPS */
            Store (One, RSTU) /* \RSTU */
        }

        If (LEqual (Arg0, 0x05))
        {
            Store (One, PWDE) /* \PWDE */
        }
    }

    Method (SWAK, 1, NotSerialized)
    {
        If (LEqual (Arg0, 0x03))
        {
            Store (One, RSTU) /* \RSTU */
        }

        Store (PEWS, PEWS) /* \PEWS */
        Store (Zero, PEWD) /* \PEWD */
        If (PICM)
        {
            \_SB.DSPI ()
        }

        If (TMSE)
        {
            Store (Zero, TMSE) /* \TMSE */
        }

        If (LEqual (Arg0, 0x03))
        {
            Notify (\_SB.PWRB, 0x02) // Device Wake
        }

        If (LEqual (Arg0, 0x04))
        {
            Notify (\_SB.PWRB, 0x02) // Device Wake
        }
    }

    Scope (_GPE)
    {
    }

    Scope (_SB)
    {
        Name (PRSA, ResourceTemplate ()
        {
            IRQ (Level, ActiveLow, Shared, )
                {4,5,7,10,11,14,15}
        })
        Alias (PRSA, PRSB)
        Alias (PRSA, PRSC)
        Alias (PRSA, PRSD)
        Alias (PRSA, PRSE)
        Alias (PRSA, PRSF)
        Alias (PRSA, PRSG)
        Alias (PRSA, PRSH)
        Name (PD10, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                LNKA, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                LNKB, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                LNKC, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                LNKD, 
                Zero
            }
        })
        Name (AR10, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                Zero, 
                0x18
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                Zero, 
                0x19
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                Zero, 
                0x1A
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                Zero, 
                0x1B
            }
        })
        Name (PD14, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                LNKE, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                LNKF, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                LNKG, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                LNKH, 
                Zero
            }
        })
        Name (AR14, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                Zero, 
                0x1C
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                Zero, 
                0x1D
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                Zero, 
                0x1E
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                Zero, 
                0x1F
            }
        })
        Name (PD18, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                LNKA, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                LNKB, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                LNKC, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                LNKD, 
                Zero
            }
        })
        Name (AR18, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                Zero, 
                0x20
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                Zero, 
                0x21
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                Zero, 
                0x22
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                Zero, 
                0x23
            }
        })
        Name (PD1C, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                LNKE, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                LNKF, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                LNKG, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                LNKH, 
                Zero
            }
        })
        Name (AR1C, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                Zero, 
                0x24
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                Zero, 
                0x25
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                Zero, 
                0x26
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                Zero, 
                0x27
            }
        })
        Name (PD24, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                LNKE, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                LNKF, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                LNKG, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                LNKH, 
                Zero
            }
        })
        Name (AR24, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                Zero, 
                0x2C
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                Zero, 
                0x2D
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                Zero, 
                0x2E
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                Zero, 
                0x2F
            }
        })
        Name (PD28, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                LNKA, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                LNKB, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                LNKC, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                LNKD, 
                Zero
            }
        })
        Name (AR28, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                Zero, 
                0x30
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                Zero, 
                0x31
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                Zero, 
                0x32
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                Zero, 
                0x33
            }
        })
        Name (PD38, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                LNKG, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                LNKH, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                LNKE, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                LNKF, 
                Zero
            }
        })
        Name (AR38, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                Zero, 
                0x26
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                Zero, 
                0x27
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                Zero, 
                0x24
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                Zero, 
                0x25
            }
        })
        Name (PD39, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                LNKC, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                LNKD, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                LNKA, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                LNKB, 
                Zero
            }
        })
        Name (AR39, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                Zero, 
                0x22
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                Zero, 
                0x23
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                Zero, 
                0x20
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                Zero, 
                0x21
            }
        })
        Name (PD3A, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                LNKG, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                LNKH, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                LNKE, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                LNKF, 
                Zero
            }
        })
        Name (AR3A, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                Zero, 
                0x1E
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                Zero, 
                0x1F
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                Zero, 
                0x1C
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                Zero, 
                0x1D
            }
        })
        Name (PD00, Package (0x0E)
        {
            Package (0x04)
            {
                0x0001FFFF, 
                Zero, 
                LNKA, 
                Zero
            }, 

            Package (0x04)
            {
                0x0001FFFF, 
                One, 
                LNKB, 
                Zero
            }, 

            Package (0x04)
            {
                0x0001FFFF, 
                0x02, 
                LNKC, 
                Zero
            }, 

            Package (0x04)
            {
                0x0002FFFF, 
                Zero, 
                LNKE, 
                Zero
            }, 

            Package (0x04)
            {
                0x0002FFFF, 
                One, 
                LNKF, 
                Zero
            }, 

            Package (0x04)
            {
                0x0002FFFF, 
                0x02, 
                LNKG, 
                Zero
            }, 

            Package (0x04)
            {
                0x0002FFFF, 
                0x03, 
                LNKH, 
                Zero
            }, 

            Package (0x04)
            {
                0x0008FFFF, 
                Zero, 
                LNKA, 
                Zero
            }, 

            Package (0x04)
            {
                0x0008FFFF, 
                One, 
                LNKB, 
                Zero
            }, 

            Package (0x04)
            {
                0x0008FFFF, 
                0x02, 
                LNKC, 
                Zero
            }, 

            Package (0x04)
            {
                0x0014FFFF, 
                Zero, 
                LNKA, 
                Zero
            }, 

            Package (0x04)
            {
                0x0014FFFF, 
                One, 
                LNKB, 
                Zero
            }, 

            Package (0x04)
            {
                0x0014FFFF, 
                0x02, 
                LNKC, 
                Zero
            }, 

            Package (0x04)
            {
                0x0014FFFF, 
                0x03, 
                LNKD, 
                Zero
            }
        })
        Name (AR00, Package (0x0E)
        {
            Package (0x04)
            {
                0x0001FFFF, 
                Zero, 
                Zero, 
                0x28
            }, 

            Package (0x04)
            {
                0x0001FFFF, 
                One, 
                Zero, 
                0x29
            }, 

            Package (0x04)
            {
                0x0001FFFF, 
                0x02, 
                Zero, 
                0x2A
            }, 

            Package (0x04)
            {
                0x0002FFFF, 
                Zero, 
                Zero, 
                0x24
            }, 

            Package (0x04)
            {
                0x0002FFFF, 
                One, 
                Zero, 
                0x25
            }, 

            Package (0x04)
            {
                0x0002FFFF, 
                0x02, 
                Zero, 
                0x26
            }, 

            Package (0x04)
            {
                0x0002FFFF, 
                0x03, 
                Zero, 
                0x27
            }, 

            Package (0x04)
            {
                0x0008FFFF, 
                Zero, 
                Zero, 
                0x20
            }, 

            Package (0x04)
            {
                0x0008FFFF, 
                One, 
                Zero, 
                0x21
            }, 

            Package (0x04)
            {
                0x0008FFFF, 
                0x02, 
                Zero, 
                0x22
            }, 

            Package (0x04)
            {
                0x0014FFFF, 
                Zero, 
                Zero, 
                0x10
            }, 

            Package (0x04)
            {
                0x0014FFFF, 
                One, 
                Zero, 
                0x11
            }, 

            Package (0x04)
            {
                0x0014FFFF, 
                0x02, 
                Zero, 
                0x12
            }, 

            Package (0x04)
            {
                0x0014FFFF, 
                0x03, 
                Zero, 
                0x13
            }
        })
        Name (PD20, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                LNKA, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                LNKB, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                LNKC, 
                Zero
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                LNKD, 
                Zero
            }
        })
        Name (AR20, Package (0x04)
        {
            Package (0x04)
            {
                0xFFFF, 
                Zero, 
                Zero, 
                0x28
            }, 

            Package (0x04)
            {
                0xFFFF, 
                One, 
                Zero, 
                0x29
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x02, 
                Zero, 
                0x2A
            }, 

            Package (0x04)
            {
                0xFFFF, 
                0x03, 
                Zero, 
                0x2B
            }
        })
    }

    Scope (_SB)
    {
        Device (PCI0)
        {
            Name (_HID, EisaId ("PNP0A08") /* PCI Express Bus */)  // _HID: Hardware ID
            Name (_CID, EisaId ("PNP0A03") /* PCI Bus */)  // _CID: Compatible ID
            Name (_ADR, Zero)  // _ADR: Address
            Method (^BN00, 0, NotSerialized)
            {
                Return (Zero)
            }

            Method (_BBN, 0, NotSerialized)  // _BBN: BIOS Bus Number
            {
                Return (BN00 ())
            }

            Name (_UID, Zero)  // _UID: Unique ID
            Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
            {
                If (PICM)
                {
                    Return (AR00) /* \_SB_.AR00 */
                }

                Return (PD00) /* \_SB_.PD00 */
            }

            Device (AMDN)
            {
                Name (_HID, EisaId ("PNP0C01") /* System Board */)  // _HID: Hardware ID
                Name (_UID, 0xC8)  // _UID: Unique ID
                Name (_STA, 0x0F)  // _STA: Status
                Name (NPTR, ResourceTemplate ()
                {
                    Memory32Fixed (ReadWrite,
                        0x00000000,         // Address Base
                        0x00000000,         // Address Length
                        _Y00)
                })
                Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                {
                    CreateDWordField (NPTR, \_SB.PCI0.AMDN._Y00._LEN, PL)  // _LEN: Length
                    CreateDWordField (NPTR, \_SB.PCI0.AMDN._Y00._BAS, PB)  // _BAS: Base Address
                    Store (PEBS, PB) /* \_SB_.PCI0.AMDN._CRS.PB__ */
                    Store (PEBL, PL) /* \_SB_.PCI0.AMDN._CRS.PL__ */
                    Return (NPTR) /* \_SB_.PCI0.AMDN.NPTR */
                }
            }

            Method (NPTS, 1, NotSerialized)
            {
                APTS (Arg0)
            }

            Method (NWAK, 1, NotSerialized)
            {
                AWAK (Arg0)
            }

            Name (CPRB, One)
            Name (LVGA, 0x01)
            Name (STAV, 0x0F)
            Name (BRB, 0x0000)
            Name (BRL, 0x0100)
            Name (IOB, 0x1000)
            Name (IOL, 0xF000)
            Name (MBB, 0xB0000000)
            Name (MBL, 0x50000000)
            Name (MAB, 0x0000000000000000)
            Name (MAL, 0x0000000000000000)
            Name (MAM, 0x0000000000000000)
            Name (CRS1, ResourceTemplate ()
            {
                WordBusNumber (ResourceProducer, MinFixed, MaxFixed, PosDecode,
                    0x0000,             // Granularity
                    0x0000,             // Range Minimum
                    0x007F,             // Range Maximum
                    0x0000,             // Translation Offset
                    0x0080,             // Length
                    ,, _Y01)
                IO (Decode16,
                    0x0CF8,             // Range Minimum
                    0x0CF8,             // Range Maximum
                    0x01,               // Alignment
                    0x08,               // Length
                    )
                WordIO (ResourceProducer, MinFixed, MaxFixed, PosDecode, EntireRange,
                    0x0000,             // Granularity
                    0x0000,             // Range Minimum
                    0x03AF,             // Range Maximum
                    0x0000,             // Translation Offset
                    0x03B0,             // Length
                    ,, , TypeStatic, DenseTranslation)
                WordIO (ResourceProducer, MinFixed, MaxFixed, PosDecode, EntireRange,
                    0x0000,             // Granularity
                    0x03E0,             // Range Minimum
                    0x0CF7,             // Range Maximum
                    0x0000,             // Translation Offset
                    0x0918,             // Length
                    ,, , TypeStatic, DenseTranslation)
                WordIO (ResourceProducer, MinFixed, MaxFixed, PosDecode, EntireRange,
                    0x0000,             // Granularity
                    0x0000,             // Range Minimum
                    0x0000,             // Range Maximum
                    0x0000,             // Translation Offset
                    0x0000,             // Length
                    ,, _Y03, TypeStatic, DenseTranslation)
                WordIO (ResourceProducer, MinFixed, MaxFixed, PosDecode, EntireRange,
                    0x0000,             // Granularity
                    0x0D00,             // Range Minimum
                    0x0FFF,             // Range Maximum
                    0x0000,             // Translation Offset
                    0x0300,             // Length
                    ,, _Y02, TypeStatic, DenseTranslation)
                DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                    0x00000000,         // Granularity
                    0x00000000,         // Range Minimum
                    0x00000000,         // Range Maximum
                    0x00000000,         // Translation Offset
                    0x00000000,         // Length
                    ,, _Y04, AddressRangeMemory, TypeStatic)
                DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, NonCacheable, ReadWrite,
                    0x00000000,         // Granularity
                    0x000C0000,         // Range Minimum
                    0x000DFFFF,         // Range Maximum
                    0x00000000,         // Translation Offset
                    0x00020000,         // Length
                    ,, , AddressRangeMemory, TypeStatic)
                DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                    0x00000000,         // Granularity
                    0x02000000,         // Range Minimum
                    0xFFDFFFFF,         // Range Maximum
                    0x00000000,         // Translation Offset
                    0xFDE00000,         // Length
                    ,, _Y05, AddressRangeMemory, TypeStatic)
                DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                    0x00000000,         // Granularity
                    0x02000000,         // Range Minimum
                    0xFFDFFFFF,         // Range Maximum
                    0x00000000,         // Translation Offset
                    0xFDE00000,         // Length
                    ,, _Y06, AddressRangeMemory, TypeStatic)
                QWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                    0x0000000000000000, // Granularity
                    0x0000000000000000, // Range Minimum
                    0x0000000000000000, // Range Maximum
                    0x0000000000000000, // Translation Offset
                    0x0000000000000000, // Length
                    ,, _Y07, AddressRangeMemory, TypeStatic)
            })
            Name (CRS2, ResourceTemplate ()
            {
                WordBusNumber (ResourceProducer, MinFixed, MaxFixed, PosDecode,
                    0x0000,             // Granularity
                    0x0080,             // Range Minimum
                    0x00FF,             // Range Maximum
                    0x0000,             // Translation Offset
                    0x0080,             // Length
                    ,, _Y08)
                WordIO (ResourceProducer, MinFixed, MaxFixed, PosDecode, EntireRange,
                    0x0000,             // Granularity
                    0x0000,             // Range Minimum
                    0x0000,             // Range Maximum
                    0x0000,             // Translation Offset
                    0x0000,             // Length
                    ,, _Y0A, TypeStatic, DenseTranslation)
                WordIO (ResourceProducer, MinFixed, MaxFixed, PosDecode, EntireRange,
                    0x0000,             // Granularity
                    0x0000,             // Range Minimum
                    0x0000,             // Range Maximum
                    0x0000,             // Translation Offset
                    0x0000,             // Length
                    ,, _Y09, TypeStatic, DenseTranslation)
                DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                    0x00000000,         // Granularity
                    0x00000000,         // Range Minimum
                    0x00000000,         // Range Maximum
                    0x00000000,         // Translation Offset
                    0x00000000,         // Length
                    ,, _Y0B, AddressRangeMemory, TypeStatic)
                DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                    0x00000000,         // Granularity
                    0x80000000,         // Range Minimum
                    0xFFFFFFFF,         // Range Maximum
                    0x00000000,         // Translation Offset
                    0x80000000,         // Length
                    ,, _Y0C, AddressRangeMemory, TypeStatic)
                DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                    0x00000000,         // Granularity
                    0x80000000,         // Range Minimum
                    0xFFFFFFFF,         // Range Maximum
                    0x00000000,         // Translation Offset
                    0x80000000,         // Length
                    ,, _Y0D, AddressRangeMemory, TypeStatic)
                QWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                    0x0000000000000000, // Granularity
                    0x0000000000000000, // Range Minimum
                    0x0000000000000000, // Range Maximum
                    0x0000000000000000, // Translation Offset
                    0x0000000000000000, // Length
                    ,, _Y0E, AddressRangeMemory, TypeStatic)
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (STAV) /* \_SB_.PCI0.STAV */
            }

            Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
            {
                Store (0x25, DBG8) /* \DBG8 */
                If (CPRB)
                {
                    CreateWordField (CRS1, \_SB.PCI0._Y01._MIN, MIN0)  // _MIN: Minimum Base Address
                    CreateWordField (CRS1, \_SB.PCI0._Y01._MAX, MAX0)  // _MAX: Maximum Base Address
                    CreateWordField (CRS1, \_SB.PCI0._Y01._LEN, LEN0)  // _LEN: Length
                    Store (BRB, MIN0) /* \_SB_.PCI0._CRS.MIN0 */
                    Store (BRL, LEN0) /* \_SB_.PCI0._CRS.LEN0 */
                    Store (LEN0, Local0)
                    Add (MIN0, Decrement (Local0), MAX0) /* \_SB_.PCI0._CRS.MAX0 */
                    CreateWordField (CRS1, \_SB.PCI0._Y02._MIN, MIN1)  // _MIN: Minimum Base Address
                    CreateWordField (CRS1, \_SB.PCI0._Y02._MAX, MAX1)  // _MAX: Maximum Base Address
                    CreateWordField (CRS1, \_SB.PCI0._Y02._LEN, LEN1)  // _LEN: Length
                    If (LEqual (IOB, 0x1000))
                    {
                        Store (IOL, Local0)
                        Add (IOB, Decrement (Local0), MAX1) /* \_SB_.PCI0._CRS.MAX1 */
                        Subtract (MAX1, MIN1, Local0)
                        Add (Local0, One, LEN1) /* \_SB_.PCI0._CRS.LEN1 */
                    }
                    Else
                    {
                        Store (IOB, MIN1) /* \_SB_.PCI0._CRS.MIN1 */
                        Store (IOL, LEN1) /* \_SB_.PCI0._CRS.LEN1 */
                        Store (LEN1, Local0)
                        Add (MIN1, Decrement (Local0), MAX1) /* \_SB_.PCI0._CRS.MAX1 */
                    }

                    If (LOr (LEqual (LVGA, One), LEqual (LVGA, 0x55)))
                    {
                        If (VGAF)
                        {
                            CreateWordField (CRS1, \_SB.PCI0._Y03._MIN, IMN1)  // _MIN: Minimum Base Address
                            CreateWordField (CRS1, \_SB.PCI0._Y03._MAX, IMX1)  // _MAX: Maximum Base Address
                            CreateWordField (CRS1, \_SB.PCI0._Y03._LEN, ILN1)  // _LEN: Length
                            Store (0x03B0, IMN1) /* \_SB_.PCI0._CRS.IMN1 */
                            Store (0x03DF, IMX1) /* \_SB_.PCI0._CRS.IMX1 */
                            Store (0x30, ILN1) /* \_SB_.PCI0._CRS.ILN1 */
                            CreateDWordField (CRS1, \_SB.PCI0._Y04._MIN, VMN1)  // _MIN: Minimum Base Address
                            CreateDWordField (CRS1, \_SB.PCI0._Y04._MAX, VMX1)  // _MAX: Maximum Base Address
                            CreateDWordField (CRS1, \_SB.PCI0._Y04._LEN, VLN1)  // _LEN: Length
                            Store (0x000A0000, VMN1) /* \_SB_.PCI0._CRS.VMN1 */
                            Store (0x000BFFFF, VMX1) /* \_SB_.PCI0._CRS.VMX1 */
                            Store (0x00020000, VLN1) /* \_SB_.PCI0._CRS.VLN1 */
                            Store (Zero, VGAF) /* \_SB_.VGAF */
                        }
                    }

                    CreateDWordField (CRS1, \_SB.PCI0._Y05._MIN, MIN3)  // _MIN: Minimum Base Address
                    CreateDWordField (CRS1, \_SB.PCI0._Y05._MAX, MAX3)  // _MAX: Maximum Base Address
                    CreateDWordField (CRS1, \_SB.PCI0._Y05._LEN, LEN3)  // _LEN: Length
                    CreateDWordField (CRS1, \_SB.PCI0._Y06._MIN, MIN7)  // _MIN: Minimum Base Address
                    CreateDWordField (CRS1, \_SB.PCI0._Y06._MAX, MAX7)  // _MAX: Maximum Base Address
                    CreateDWordField (CRS1, \_SB.PCI0._Y06._LEN, LEN7)  // _LEN: Length
                    Add (MBB, MBL, Local0)
                    If (LLess (Local0, NBTP))
                    {
                        Store (MBB, MIN3) /* \_SB_.PCI0._CRS.MIN3 */
                        Store (MBL, LEN3) /* \_SB_.PCI0._CRS.LEN3 */
                        Store (LEN3, Local0)
                        Add (MIN3, Decrement (Local0), MAX3) /* \_SB_.PCI0._CRS.MAX3 */
                        Store (Zero, MIN7) /* \_SB_.PCI0._CRS.MIN7 */
                        Store (Zero, MAX7) /* \_SB_.PCI0._CRS.MAX7 */
                        Store (Zero, LEN7) /* \_SB_.PCI0._CRS.LEN7 */
                    }
                    Else
                    {
                        Store (MBB, MIN3) /* \_SB_.PCI0._CRS.MIN3 */
                        Store (Subtract (NBTP, MBB), LEN3) /* \_SB_.PCI0._CRS.LEN3 */
                        Store (LEN3, Local0)
                        Add (MIN3, Decrement (Local0), MAX3) /* \_SB_.PCI0._CRS.MAX3 */
                        Store (0xFEE00000, MIN7) /* \_SB_.PCI0._CRS.MIN7 */
                        Store (Subtract (0xFEE00000, NBTP), Local0)
                        Store (Subtract (MBL, Local0), LEN7) /* \_SB_.PCI0._CRS.LEN7 */
                        Store (Subtract (LEN7, LEN3), LEN7) /* \_SB_.PCI0._CRS.LEN7 */
                        Store (LEN7, Local0)
                        Add (MIN7, Decrement (Local0), MAX7) /* \_SB_.PCI0._CRS.MAX7 */
                    }

                    If (MAL)
                    {
                        CreateQWordField (CRS1, \_SB.PCI0._Y07._MIN, MN8)  // _MIN: Minimum Base Address
                        CreateQWordField (CRS1, \_SB.PCI0._Y07._MAX, MX8)  // _MAX: Maximum Base Address
                        CreateQWordField (CRS1, \_SB.PCI0._Y07._LEN, LN8)  // _LEN: Length
                        Store (MAB, MN8) /* \_SB_.PCI0._CRS.MN8_ */
                        Store (MAL, LN8) /* \_SB_.PCI0._CRS.LN8_ */
                        Store (MAM, MX8) /* \_SB_.PCI0._CRS.MX8_ */
                    }

                    Store (0x24, DBG8) /* \DBG8 */
                    Return (CRS1) /* \_SB_.PCI0.CRS1 */
                }
                Else
                {
                    CreateWordField (CRS2, \_SB.PCI0._Y08._MIN, MIN2)  // _MIN: Minimum Base Address
                    CreateWordField (CRS2, \_SB.PCI0._Y08._MAX, MAX2)  // _MAX: Maximum Base Address
                    CreateWordField (CRS2, \_SB.PCI0._Y08._LEN, LEN2)  // _LEN: Length
                    Store (BRB, MIN2) /* \_SB_.PCI0._CRS.MIN2 */
                    Store (BRL, LEN2) /* \_SB_.PCI0._CRS.LEN2 */
                    Store (LEN2, Local1)
                    Add (MIN2, Decrement (Local1), MAX2) /* \_SB_.PCI0._CRS.MAX2 */
                    CreateWordField (CRS2, \_SB.PCI0._Y09._MIN, MIN4)  // _MIN: Minimum Base Address
                    CreateWordField (CRS2, \_SB.PCI0._Y09._MAX, MAX4)  // _MAX: Maximum Base Address
                    CreateWordField (CRS2, \_SB.PCI0._Y09._LEN, LEN4)  // _LEN: Length
                    Store (IOB, MIN4) /* \_SB_.PCI0._CRS.MIN4 */
                    Store (IOL, LEN4) /* \_SB_.PCI0._CRS.LEN4 */
                    Store (LEN4, Local1)
                    Add (MIN4, Decrement (Local1), MAX4) /* \_SB_.PCI0._CRS.MAX4 */
                    If (LVGA)
                    {
                        CreateWordField (CRS2, \_SB.PCI0._Y0A._MIN, IMN2)  // _MIN: Minimum Base Address
                        CreateWordField (CRS2, \_SB.PCI0._Y0A._MAX, IMX2)  // _MAX: Maximum Base Address
                        CreateWordField (CRS2, \_SB.PCI0._Y0A._LEN, ILN2)  // _LEN: Length
                        Store (0x03B0, IMN2) /* \_SB_.PCI0._CRS.IMN2 */
                        Store (0x03DF, IMX2) /* \_SB_.PCI0._CRS.IMX2 */
                        Store (0x30, ILN2) /* \_SB_.PCI0._CRS.ILN2 */
                        CreateDWordField (CRS2, \_SB.PCI0._Y0B._MIN, VMN2)  // _MIN: Minimum Base Address
                        CreateDWordField (CRS2, \_SB.PCI0._Y0B._MAX, VMX2)  // _MAX: Maximum Base Address
                        CreateDWordField (CRS2, \_SB.PCI0._Y0B._LEN, VLN2)  // _LEN: Length
                        Store (0x000A0000, VMN2) /* \_SB_.PCI0._CRS.VMN2 */
                        Store (0x000BFFFF, VMX2) /* \_SB_.PCI0._CRS.VMX2 */
                        Store (0x00020000, VLN2) /* \_SB_.PCI0._CRS.VLN2 */
                    }

                    CreateDWordField (CRS2, \_SB.PCI0._Y0C._MIN, MIN5)  // _MIN: Minimum Base Address
                    CreateDWordField (CRS2, \_SB.PCI0._Y0C._MAX, MAX5)  // _MAX: Maximum Base Address
                    CreateDWordField (CRS2, \_SB.PCI0._Y0C._LEN, LEN5)  // _LEN: Length
                    Store (MBB, MIN5) /* \_SB_.PCI0._CRS.MIN5 */
                    Store (Subtract (NBTP, MBB), LEN5) /* \_SB_.PCI0._CRS.LEN5 */
                    Store (LEN5, Local1)
                    Add (MIN5, Decrement (Local1), MAX5) /* \_SB_.PCI0._CRS.MAX5 */
                    CreateDWordField (CRS2, \_SB.PCI0._Y0D._MIN, MIN6)  // _MIN: Minimum Base Address
                    CreateDWordField (CRS2, \_SB.PCI0._Y0D._MAX, MAX6)  // _MAX: Maximum Base Address
                    CreateDWordField (CRS2, \_SB.PCI0._Y0D._LEN, LEN6)  // _LEN: Length
                    Add (NBTP, NBTS, MIN6) /* \_SB_.PCI0._CRS.MIN6 */
                    Store (Subtract (MBL, NBTS), LEN6) /* \_SB_.PCI0._CRS.LEN6 */
                    Store (Subtract (LEN6, LEN5), LEN6) /* \_SB_.PCI0._CRS.LEN6 */
                    Store (LEN6, Local0)
                    Add (MIN6, Decrement (Local0), MAX6) /* \_SB_.PCI0._CRS.MAX6 */
                    If (MAL)
                    {
                        CreateQWordField (CRS2, \_SB.PCI0._Y0E._MIN, MN9)  // _MIN: Minimum Base Address
                        CreateQWordField (CRS2, \_SB.PCI0._Y0E._MAX, MX9)  // _MAX: Maximum Base Address
                        CreateQWordField (CRS2, \_SB.PCI0._Y0E._LEN, LN9)  // _LEN: Length
                        Store (MAB, MN9) /* \_SB_.PCI0._CRS.MN9_ */
                        Store (MAL, LN9) /* \_SB_.PCI0._CRS.LN9_ */
                        Store (MAM, MX9) /* \_SB_.PCI0._CRS.MX9_ */
                    }

                    Store (0x23, DBG8) /* \DBG8 */
                    Return (CRS2) /* \_SB_.PCI0.CRS2 */
                }
            }

            Method (_OSC, 4, Serialized)  // _OSC: Operating System Capabilities
            {
                Name (SUPP, Zero)
                Name (CTRL, Zero)
                CreateDWordField (Arg3, Zero, CDW1)
                CreateDWordField (Arg3, 0x04, CDW2)
                CreateDWordField (Arg3, 0x08, CDW3)
                If (LEqual (Arg0, ToUUID ("33db4d5b-1ff7-401c-9657-7441c03dd766") /* PCI Host Bridge Device */))
                {
                    Store (CDW2, SUPP) /* \_SB_.PCI0._OSC.SUPP */
                    Store (CDW3, CTRL) /* \_SB_.PCI0._OSC.CTRL */
                    If (LNotEqual (And (SUPP, 0x16), 0x16))
                    {
                        And (CTRL, 0x1E, CTRL) /* \_SB_.PCI0._OSC.CTRL */
                    }

                    If (LNot (PEHP))
                    {
                        And (CTRL, 0x1E, CTRL) /* \_SB_.PCI0._OSC.CTRL */
                    }

                    If (LNot (SHPC))
                    {
                        And (CTRL, 0x1D, CTRL) /* \_SB_.PCI0._OSC.CTRL */
                    }

                    If (LNot (PEPM))
                    {
                        And (CTRL, 0x1B, CTRL) /* \_SB_.PCI0._OSC.CTRL */
                    }

                    If (LNot (PEER))
                    {
                        And (CTRL, 0x15, CTRL) /* \_SB_.PCI0._OSC.CTRL */
                    }

                    If (LNot (PECS))
                    {
                        And (CTRL, 0x0F, CTRL) /* \_SB_.PCI0._OSC.CTRL */
                    }

                    If (LNotEqual (Arg1, One))
                    {
                        Or (CDW1, 0x08, CDW1) /* \_SB_.PCI0._OSC.CDW1 */
                    }

                    If (LNotEqual (CDW3, CTRL))
                    {
                        Or (CDW1, 0x10, CDW1) /* \_SB_.PCI0._OSC.CDW1 */
                    }

                    Store (CTRL, CDW3) /* \_SB_.PCI0._OSC.CDW3 */
                    Return (Arg3)
                }
                Else
                {
                    Or (CDW1, 0x04, CDW1) /* \_SB_.PCI0._OSC.CDW1 */
                    Return (Arg3)
                }
            }

            Mutex (NAPM, 0x00)
            Method (NAPE, 0, NotSerialized)
            {
                Acquire (NAPM, 0xFFFF)
                Store (0x11, DBG8) /* \DBG8 */
                Add (PEBS, 0xB8, Local0)
                OperationRegion (VARM, SystemMemory, Local0, 0x08)
                Field (VARM, DWordAcc, NoLock, Preserve)
                {
                    NAPX,   32, 
                    NAPD,   32
                }

                Store (NAPX, Local1)
                Store (0x14300000, NAPX) /* \_SB_.PCI0.NAPE.NAPX */
                Store (NAPD, Local0)
                And (Local0, 0xFFFFFFEF, Local0)
                Store (Local0, NAPD) /* \_SB_.PCI0.NAPE.NAPD */
                Store (Local1, NAPX) /* \_SB_.PCI0.NAPE.NAPX */
                Store (0x12, DBG8) /* \DBG8 */
                Release (NAPM)
            }

            Device (IOMA)
            {
                Name (_ADR, 0x02)  // _ADR: Address
                Name (_HID, EisaId ("PNP0C02") /* PNP Motherboard Resources */)  // _HID: Hardware ID
                Name (_UID, 0x15)  // _UID: Unique ID
                Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                {
                    Memory32Fixed (ReadOnly,
                        0xFEB80000,         // Address Base
                        0x00080000,         // Address Length
                        )
                })
            }

            Device (D003)
            {
                Name (_ADR, Zero)  // _ADR: Address
            }

            Device (GPP0)
            {
                Name (_ADR, 0x00010001)  // _ADR: Address
                Method (_PRW, 0, NotSerialized)  // _PRW: Power Resources for Wake
                {
                    Return (GPRW (0x08, 0x04))
                }

                Device (HDAU)
                {
                    Name (_ADR, One)  // _ADR: Address
                    Method (_RMV, 0, NotSerialized)  // _RMV: Removal Status
                    {
                        Return (Zero)
                    }
                }

                Device (NXHC)
                {
                    Name (_ADR, 0x02)  // _ADR: Address
                    Method (_RMV, 0, NotSerialized)  // _RMV: Removal Status
                    {
                        Return (Zero)
                    }

                    Device (NHUB)
                    {
                        Name (_ADR, Zero)  // _ADR: Address
                        Device (SS01)
                        {
                            Name (_ADR, One)  // _ADR: Address
                            Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                            {
                                Name (NUPC, Package (0x04)
                                {
                                    Zero, 
                                    0xFF, 
                                    Zero, 
                                    Zero
                                })
                                Return (NUPC) /* \_SB_.PCI0.GPP0.NXHC.NHUB.SS01._UPC.NUPC */
                            }

                            Method (_PLD, 0, NotSerialized)  // _PLD: Physical Location of Device
                            {
                                Return (NPLD (Zero, One))
                            }
                        }

                        Device (SS02)
                        {
                            Name (_ADR, 0x02)  // _ADR: Address
                            Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                            {
                                Name (NUPC, Package (0x04)
                                {
                                    Zero, 
                                    0xFF, 
                                    Zero, 
                                    Zero
                                })
                                Return (NUPC) /* \_SB_.PCI0.GPP0.NXHC.NHUB.SS02._UPC.NUPC */
                            }

                            Method (_PLD, 0, NotSerialized)  // _PLD: Physical Location of Device
                            {
                                Return (NPLD (Zero, 0x02))
                            }
                        }

                        Device (SS03)
                        {
                            Name (_ADR, 0x03)  // _ADR: Address
                            Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                            {
                                Name (NUPC, Package (0x04)
                                {
                                    Zero, 
                                    0xFF, 
                                    Zero, 
                                    Zero
                                })
                                Return (NUPC) /* \_SB_.PCI0.GPP0.NXHC.NHUB.SS03._UPC.NUPC */
                            }

                            Method (_PLD, 0, NotSerialized)  // _PLD: Physical Location of Device
                            {
                                Return (NPLD (Zero, 0x03))
                            }
                        }

                        Device (SS04)
                        {
                            Name (_ADR, 0x04)  // _ADR: Address
                            Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                            {
                                Name (NUPC, Package (0x04)
                                {
                                    Zero, 
                                    0xFF, 
                                    Zero, 
                                    Zero
                                })
                                Return (NUPC) /* \_SB_.PCI0.GPP0.NXHC.NHUB.SS04._UPC.NUPC */
                            }

                            Method (_PLD, 0, NotSerialized)  // _PLD: Physical Location of Device
                            {
                                Return (NPLD (Zero, 0x04))
                            }
                        }

                        Device (SS05)
                        {
                            Name (_ADR, 0x05)  // _ADR: Address
                            Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                            {
                                Name (NUPC, Package (0x04)
                                {
                                    Zero, 
                                    0xFF, 
                                    Zero, 
                                    Zero
                                })
                                Return (NUPC) /* \_SB_.PCI0.GPP0.NXHC.NHUB.SS05._UPC.NUPC */
                            }

                            Method (_PLD, 0, NotSerialized)  // _PLD: Physical Location of Device
                            {
                                Return (NPLD (Zero, 0x05))
                            }
                        }

                        Device (SS06)
                        {
                            Name (_ADR, 0x06)  // _ADR: Address
                            Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                            {
                                Name (NUPC, Package (0x04)
                                {
                                    Zero, 
                                    0xFF, 
                                    Zero, 
                                    Zero
                                })
                                Return (NUPC) /* \_SB_.PCI0.GPP0.NXHC.NHUB.SS06._UPC.NUPC */
                            }

                            Method (_PLD, 0, NotSerialized)  // _PLD: Physical Location of Device
                            {
                                Return (NPLD (Zero, 0x06))
                            }
                        }
                    }

                    Method (NPLD, 2, Serialized)
                    {
                        Name (PCKG, Package (0x01)
                        {
                            Buffer (0x10){}
                        })
                        CreateField (DerefOf (Index (PCKG, Zero)), Zero, 0x07, REV)
                        Store (One, REV) /* \_SB_.PCI0.GPP0.NXHC.NPLD.REV_ */
                        CreateField (DerefOf (Index (PCKG, Zero)), 0x40, One, VISI)
                        Store (Arg0, VISI) /* \_SB_.PCI0.GPP0.NXHC.NPLD.VISI */
                        CreateField (DerefOf (Index (PCKG, Zero)), 0x57, 0x08, GPOS)
                        Store (Arg1, GPOS) /* \_SB_.PCI0.GPP0.NXHC.NPLD.GPOS */
                        Return (PCKG) /* \_SB_.PCI0.GPP0.NXHC.NPLD.PCKG */
                    }
                }

                Device (USBC)
                {
                    Name (_ADR, 0x03)  // _ADR: Address
                    Method (_RMV, 0, NotSerialized)  // _RMV: Removal Status
                    {
                        Return (Zero)
                    }
                }

                Method (MPRW, 0, NotSerialized)
                {
                    Return (GPRW (0x08, Zero))
                }

                Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
                {
                    If (PICM)
                    {
                        Return (AR10) /* \_SB_.AR10 */
                    }

                    Return (PD10) /* \_SB_.PD10 */
                }

                Device (D036)
                {
                    Name (_ADR, 0xFF)  // _ADR: Address
                }
            }

            Device (GPP1)
            {
                Name (_ADR, 0x00010002)  // _ADR: Address
                Method (_PRW, 0, NotSerialized)  // _PRW: Power Resources for Wake
                {
                    Return (GPRW (0x08, 0x04))
                }

                Device (DEV0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Method (_RMV, 0, NotSerialized)  // _RMV: Removal Status
                    {
                        Return (Zero)
                    }
                }

                Device (DEV1)
                {
                    Name (_ADR, One)  // _ADR: Address
                    Method (_RMV, 0, NotSerialized)  // _RMV: Removal Status
                    {
                        Return (Zero)
                    }
                }

                Method (MPRW, 0, NotSerialized)
                {
                    Return (GPRW (0x08, Zero))
                }

                Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
                {
                    If (PICM)
                    {
                        Return (AR14) /* \_SB_.AR14 */
                    }

                    Return (PD14) /* \_SB_.PD14 */
                }

                Device (D006)
                {
                    Name (_ADR, 0xFF)  // _ADR: Address
                }
            }

            Device (GPP2)
            {
                Name (_ADR, 0x00010003)  // _ADR: Address
                Method (_PRW, 0, NotSerialized)  // _PRW: Power Resources for Wake
                {
                    Return (GPRW (0x0D, 0x04))
                }

                Method (MPRW, 0, NotSerialized)
                {
                    Return (GPRW (0x0D, Zero))
                }

                Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
                {
                    If (PICM)
                    {
                        Return (AR18) /* \_SB_.AR18 */
                    }

                    Return (PD18) /* \_SB_.PD18 */
                }

                Device (D008)
                {
                    Name (_ADR, 0xFF)  // _ADR: Address
                }
            }

            Device (GPP3)
            {
                Name (_ADR, 0x00020001)  // _ADR: Address
                Method (_PRW, 0, NotSerialized)  // _PRW: Power Resources for Wake
                {
                    Return (GPRW (0x0F, 0x04))
                }

                Method (MPRW, 0, NotSerialized)
                {
                    Return (GPRW (0x0F, Zero))
                }

                Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
                {
                    If (PICM)
                    {
                        Return (AR1C) /* \_SB_.AR1C */
                    }

                    Return (PD1C) /* \_SB_.PD1C */
                }

                Device (RTL8)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                }

                Device (RUSB)
                {
                    Name (_ADR, 0x04)  // _ADR: Address
                }
            }

            Device (GPP5)
            {
                Name (_ADR, 0x00020003)  // _ADR: Address
                Method (_PRW, 0, NotSerialized)  // _PRW: Power Resources for Wake
                {
                    Return (GPRW (0x08, 0x04))
                }

                Method (MPRW, 0, NotSerialized)
                {
                    Return (GPRW (0x08, Zero))
                }

                Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
                {
                    If (PICM)
                    {
                        Return (AR24) /* \_SB_.AR24 */
                    }

                    Return (PD24) /* \_SB_.PD24 */
                }

                Device (D00B)
                {
                    Name (_ADR, 0xFF)  // _ADR: Address
                }
            }

            Device (GPP6)
            {
                Name (_ADR, 0x00020004)  // _ADR: Address
                Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
                {
                    If (PICM)
                    {
                        Return (AR28) /* \_SB_.AR28 */
                    }

                    Return (PD28) /* \_SB_.PD28 */
                }

                Device (D00D)
                {
                    Name (_ADR, 0xFF)  // _ADR: Address
                }
            }

            Device (GP17)
            {
                Name (_ADR, 0x00080001)  // _ADR: Address
                Method (_PRW, 0, NotSerialized)  // _PRW: Power Resources for Wake
                {
                    Return (GPRW (0x19, 0x04))
                }

                Method (MPRW, 0, NotSerialized)
                {
                    Return (GPRW (0x19, Zero))
                }

                Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
                {
                    If (PICM)
                    {
                        Return (AR38) /* \_SB_.AR38 */
                    }

                    Return (PD38) /* \_SB_.PD38 */
                }

                Device (VGA)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (AF7E, 0x80000001)
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (0x0F)
                    }

                    Name (DOSA, Zero)
                    Method (_DOS, 1, NotSerialized)  // _DOS: Disable Output Switching
                    {
                        Store (Arg0, DOSA) /* \_SB_.PCI0.GP17.VGA_.DOSA */
                    }

                    Method (_DOD, 0, NotSerialized)  // _DOD: Display Output Devices
                    {
                        Return (Package (0x07)
                        {
                            0x00010110, 
                            0x00010210, 
                            0x00010220, 
                            0x00010230, 
                            0x00010240, 
                            0x00031000, 
                            0x00032000
                        })
                    }

                    Device (LCD)
                    {
                        Name (_ADR, 0x0110)  // _ADR: Address
                        Name (BCLB, Package (0x34)
                        {
                            0x5A, 
                            0x3C, 
                            0x02, 
                            0x04, 
                            0x06, 
                            0x08, 
                            0x0A, 
                            0x0C, 
                            0x0E, 
                            0x10, 
                            0x12, 
                            0x14, 
                            0x16, 
                            0x18, 
                            0x1A, 
                            0x1C, 
                            0x1E, 
                            0x20, 
                            0x22, 
                            0x24, 
                            0x26, 
                            0x28, 
                            0x2A, 
                            0x2C, 
                            0x2E, 
                            0x30, 
                            0x32, 
                            0x34, 
                            0x36, 
                            0x38, 
                            0x3A, 
                            0x3C, 
                            0x3E, 
                            0x40, 
                            0x42, 
                            0x44, 
                            0x46, 
                            0x48, 
                            0x4A, 
                            0x4C, 
                            0x4E, 
                            0x50, 
                            0x52, 
                            0x54, 
                            0x56, 
                            0x58, 
                            0x5A, 
                            0x5C, 
                            0x5E, 
                            0x60, 
                            0x62, 
                            0x64
                        })
                        Method (_BCL, 0, NotSerialized)  // _BCL: Brightness Control Levels
                        {
                            Return (BCLB) /* \_SB_.PCI0.GP17.VGA_.LCD_.BCLB */
                        }

                        Method (_BCM, 1, NotSerialized)  // _BCM: Brightness Control Method
                        {
                            If (LEqual (AF7E, 0x80000001))
                            {
                                Divide (Multiply (Arg0, 0xFF), 0x64, Local1, Local0)
                                AFN7 (Local0)
                            }
                        }
                    }
                }

                Device (HDAU)
                {
                    Name (_ADR, One)  // _ADR: Address
                }

                Device (ACP)
                {
                    Name (_ADR, 0x05)  // _ADR: Address
                }

                Device (AZAL)
                {
                    Name (_ADR, 0x06)  // _ADR: Address
                }

                Device (MP2C)
                {
                    Name (_ADR, 0x07)  // _ADR: Address
                }

                Device (XHC0)
                {
                    Name (_ADR, 0x03)  // _ADR: Address
                    Method (_PRW, 0, NotSerialized)  // _PRW: Power Resources for Wake
                    {
                        Return (GPRW (0x19, 0x03))
                    }

                    Method (MPRW, 0, NotSerialized)
                    {
                        Return (GPRW (0x19, Zero))
                    }
                }

                Device (XHC1)
                {
                    Name (_ADR, 0x04)  // _ADR: Address
                    Method (_PRW, 0, NotSerialized)  // _PRW: Power Resources for Wake
                    {
                        Return (GPRW (0x19, 0x03))
                    }

                    Method (MPRW, 0, NotSerialized)
                    {
                        Return (GPRW (0x19, Zero))
                    }
                }

                Device (APSP)
                {
                    Name (_ADR, 0x02)  // _ADR: Address
                    Name (_HID, EisaId ("PNP0C02") /* PNP Motherboard Resources */)  // _HID: Hardware ID
                    Name (PSPA, 0xFD000000)
                    Name (LENA, 0x01000000)
                    Name (PSPB, 0x00000000)
                    Name (LENB, 0x00000000)
                    Name (_STA, 0x0F)  // _STA: Status
                    Name (CRS, ResourceTemplate ()
                    {
                        Memory32Fixed (ReadWrite,
                            0x00000000,         // Address Base
                            0x00000000,         // Address Length
                            _Y0F)
                        Memory32Fixed (ReadWrite,
                            0x00000000,         // Address Base
                            0x00000000,         // Address Length
                            _Y10)
                    })
                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        CreateDWordField (CRS, \_SB.PCI0.GP17.APSP._Y0F._BAS, ABAS)  // _BAS: Base Address
                        CreateDWordField (CRS, \_SB.PCI0.GP17.APSP._Y0F._LEN, ALEN)  // _LEN: Length
                        CreateDWordField (CRS, \_SB.PCI0.GP17.APSP._Y10._BAS, BBAS)  // _BAS: Base Address
                        CreateDWordField (CRS, \_SB.PCI0.GP17.APSP._Y10._LEN, BLEN)  // _LEN: Length
                        Store (PSPA, ABAS) /* \_SB_.PCI0.GP17.APSP._CRS.ABAS */
                        Store (LENA, ALEN) /* \_SB_.PCI0.GP17.APSP._CRS.ALEN */
                        Store (PSPB, BBAS) /* \_SB_.PCI0.GP17.APSP._CRS.BBAS */
                        Store (LENB, BLEN) /* \_SB_.PCI0.GP17.APSP._CRS.BLEN */
                        Return (CRS) /* \_SB_.PCI0.GP17.APSP.CRS_ */
                    }
                }
            }

            Device (GP18)
            {
                Name (_ADR, 0x00080002)  // _ADR: Address
                Method (_PRW, 0, NotSerialized)  // _PRW: Power Resources for Wake
                {
                    Return (GPRW (0x08, 0x04))
                }

                Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
                {
                    If (PICM)
                    {
                        Return (AR39) /* \_SB_.AR39 */
                    }

                    Return (PD39) /* \_SB_.PD39 */
                }

                Device (SATA)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                }

                Device (SAT1)
                {
                    Name (_ADR, One)  // _ADR: Address
                }
            }

            Device (GP19)
            {
                Name (_ADR, 0x00080003)  // _ADR: Address
                Method (_PRW, 0, NotSerialized)  // _PRW: Power Resources for Wake
                {
                    Return (GPRW (0x08, 0x04))
                }

                Method (MPRW, 0, NotSerialized)
                {
                    Return (GPRW (0x08, Zero))
                }

                Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
                {
                    If (PICM)
                    {
                        Return (AR3A) /* \_SB_.AR3A */
                    }

                    Return (PD3A) /* \_SB_.PD3A */
                }

                Device (D024)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                }

                Device (D025)
                {
                    Name (_ADR, One)  // _ADR: Address
                }

                Device (D026)
                {
                    Name (_ADR, 0x02)  // _ADR: Address
                }
            }

            Device (D01C)
            {
                Name (_ADR, 0x00140000)  // _ADR: Address
            }

            Device (SBRG)
            {
                Name (_ADR, 0x00140003)  // _ADR: Address
                Device (PIC)
                {
                    Name (_HID, EisaId ("PNP0000") /* 8259-compatible Programmable Interrupt Controller */)  // _HID: Hardware ID
                    Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                    {
                        IO (Decode16,
                            0x0020,             // Range Minimum
                            0x0020,             // Range Maximum
                            0x00,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x00A0,             // Range Minimum
                            0x00A0,             // Range Maximum
                            0x00,               // Alignment
                            0x02,               // Length
                            )
                        IRQNoFlags ()
                            {2}
                    })
                }

                Device (DMAD)
                {
                    Name (_HID, EisaId ("PNP0200") /* PC-class DMA Controller */)  // _HID: Hardware ID
                    Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                    {
                        DMA (Compatibility, BusMaster, Transfer8, )
                            {4}
                        IO (Decode16,
                            0x0000,             // Range Minimum
                            0x0000,             // Range Maximum
                            0x00,               // Alignment
                            0x10,               // Length
                            )
                        IO (Decode16,
                            0x0081,             // Range Minimum
                            0x0081,             // Range Maximum
                            0x00,               // Alignment
                            0x03,               // Length
                            )
                        IO (Decode16,
                            0x0087,             // Range Minimum
                            0x0087,             // Range Maximum
                            0x00,               // Alignment
                            0x01,               // Length
                            )
                        IO (Decode16,
                            0x0089,             // Range Minimum
                            0x0089,             // Range Maximum
                            0x00,               // Alignment
                            0x03,               // Length
                            )
                        IO (Decode16,
                            0x008F,             // Range Minimum
                            0x008F,             // Range Maximum
                            0x00,               // Alignment
                            0x01,               // Length
                            )
                        IO (Decode16,
                            0x00C0,             // Range Minimum
                            0x00C0,             // Range Maximum
                            0x00,               // Alignment
                            0x20,               // Length
                            )
                    })
                }

                Device (TMR)
                {
                    Name (_HID, EisaId ("PNP0100") /* PC-class System Timer */)  // _HID: Hardware ID
                    Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                    {
                        IO (Decode16,
                            0x0040,             // Range Minimum
                            0x0040,             // Range Maximum
                            0x00,               // Alignment
                            0x04,               // Length
                            )
                        IRQNoFlags ()
                            {0}
                    })
                }

                Device (RTC0)
                {
                    Name (_HID, EisaId ("PNP0B00") /* AT Real-Time Clock */)  // _HID: Hardware ID
                    Name (BUF0, ResourceTemplate ()
                    {
                        IO (Decode16,
                            0x0070,             // Range Minimum
                            0x0070,             // Range Maximum
                            0x00,               // Alignment
                            0x02,               // Length
                            )
                    })
                    Name (BUF1, ResourceTemplate ()
                    {
                        IO (Decode16,
                            0x0070,             // Range Minimum
                            0x0070,             // Range Maximum
                            0x00,               // Alignment
                            0x02,               // Length
                            )
                        IRQNoFlags ()
                            {8}
                    })
                    Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                    {
                        If (LEqual (HPEN, One))
                        {
                            Return (BUF0) /* \_SB_.PCI0.SBRG.RTC0.BUF0 */
                        }

                        Return (BUF1) /* \_SB_.PCI0.SBRG.RTC0.BUF1 */
                    }
                }

                Device (SPKR)
                {
                    Name (_HID, EisaId ("PNP0800") /* Microsoft Sound System Compatible Device */)  // _HID: Hardware ID
                    Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                    {
                        IO (Decode16,
                            0x0061,             // Range Minimum
                            0x0061,             // Range Maximum
                            0x00,               // Alignment
                            0x01,               // Length
                            )
                    })
                }

                OperationRegion (SMI0, SystemIO, SMIO, One)
                Field (SMI0, ByteAcc, NoLock, Preserve)
                {
                    SMIC,   8
                }

                Scope (\_SB)
                {
                    Scope (PCI0)
                    {
                        Device (S900)
                        {
                            Name (_HID, EisaId ("PNP0C02") /* PNP Motherboard Resources */)  // _HID: Hardware ID
                            Name (_UID, 0x0700)  // _UID: Unique ID
                            Name (_STA, 0x0F)  // _STA: Status
                            Name (CRS, ResourceTemplate ()
                            {
                                IO (Decode16,
                                    0x0010,             // Range Minimum
                                    0x0010,             // Range Maximum
                                    0x00,               // Alignment
                                    0x10,               // Length
                                    )
                                IO (Decode16,
                                    0x0022,             // Range Minimum
                                    0x0022,             // Range Maximum
                                    0x00,               // Alignment
                                    0x1E,               // Length
                                    )
                                IO (Decode16,
                                    0x0063,             // Range Minimum
                                    0x0063,             // Range Maximum
                                    0x00,               // Alignment
                                    0x01,               // Length
                                    )
                                IO (Decode16,
                                    0x0065,             // Range Minimum
                                    0x0065,             // Range Maximum
                                    0x00,               // Alignment
                                    0x01,               // Length
                                    )
                                IO (Decode16,
                                    0x0067,             // Range Minimum
                                    0x0067,             // Range Maximum
                                    0x00,               // Alignment
                                    0x09,               // Length
                                    )
                                IO (Decode16,
                                    0x0072,             // Range Minimum
                                    0x0072,             // Range Maximum
                                    0x00,               // Alignment
                                    0x0E,               // Length
                                    )
                                IO (Decode16,
                                    0x0080,             // Range Minimum
                                    0x0080,             // Range Maximum
                                    0x00,               // Alignment
                                    0x01,               // Length
                                    )
                                IO (Decode16,
                                    0x0084,             // Range Minimum
                                    0x0084,             // Range Maximum
                                    0x00,               // Alignment
                                    0x03,               // Length
                                    )
                                IO (Decode16,
                                    0x0088,             // Range Minimum
                                    0x0088,             // Range Maximum
                                    0x00,               // Alignment
                                    0x01,               // Length
                                    )
                                IO (Decode16,
                                    0x008C,             // Range Minimum
                                    0x008C,             // Range Maximum
                                    0x00,               // Alignment
                                    0x03,               // Length
                                    )
                                IO (Decode16,
                                    0x0090,             // Range Minimum
                                    0x0090,             // Range Maximum
                                    0x00,               // Alignment
                                    0x10,               // Length
                                    )
                                IO (Decode16,
                                    0x00A2,             // Range Minimum
                                    0x00A2,             // Range Maximum
                                    0x00,               // Alignment
                                    0x1E,               // Length
                                    )
                                IO (Decode16,
                                    0x00B1,             // Range Minimum
                                    0x00B1,             // Range Maximum
                                    0x00,               // Alignment
                                    0x01,               // Length
                                    )
                                IO (Decode16,
                                    0x00E0,             // Range Minimum
                                    0x00E0,             // Range Maximum
                                    0x00,               // Alignment
                                    0x10,               // Length
                                    )
                                IO (Decode16,
                                    0x04D0,             // Range Minimum
                                    0x04D0,             // Range Maximum
                                    0x00,               // Alignment
                                    0x02,               // Length
                                    )
                                IO (Decode16,
                                    0x040B,             // Range Minimum
                                    0x040B,             // Range Maximum
                                    0x00,               // Alignment
                                    0x01,               // Length
                                    )
                                IO (Decode16,
                                    0x04D6,             // Range Minimum
                                    0x04D6,             // Range Maximum
                                    0x00,               // Alignment
                                    0x01,               // Length
                                    )
                                IO (Decode16,
                                    0x0C00,             // Range Minimum
                                    0x0C00,             // Range Maximum
                                    0x00,               // Alignment
                                    0x02,               // Length
                                    )
                                IO (Decode16,
                                    0x0C14,             // Range Minimum
                                    0x0C14,             // Range Maximum
                                    0x00,               // Alignment
                                    0x01,               // Length
                                    )
                                IO (Decode16,
                                    0x0C50,             // Range Minimum
                                    0x0C50,             // Range Maximum
                                    0x00,               // Alignment
                                    0x02,               // Length
                                    )
                                IO (Decode16,
                                    0x0C52,             // Range Minimum
                                    0x0C52,             // Range Maximum
                                    0x00,               // Alignment
                                    0x01,               // Length
                                    )
                                IO (Decode16,
                                    0x0C6C,             // Range Minimum
                                    0x0C6C,             // Range Maximum
                                    0x00,               // Alignment
                                    0x01,               // Length
                                    )
                                IO (Decode16,
                                    0x0C6F,             // Range Minimum
                                    0x0C6F,             // Range Maximum
                                    0x00,               // Alignment
                                    0x01,               // Length
                                    )
                                IO (Decode16,
                                    0x0CD0,             // Range Minimum
                                    0x0CD0,             // Range Maximum
                                    0x00,               // Alignment
                                    0x02,               // Length
                                    )
                                IO (Decode16,
                                    0x0CD2,             // Range Minimum
                                    0x0CD2,             // Range Maximum
                                    0x00,               // Alignment
                                    0x02,               // Length
                                    )
                                IO (Decode16,
                                    0x0CD4,             // Range Minimum
                                    0x0CD4,             // Range Maximum
                                    0x00,               // Alignment
                                    0x02,               // Length
                                    )
                                IO (Decode16,
                                    0x0CD6,             // Range Minimum
                                    0x0CD6,             // Range Maximum
                                    0x00,               // Alignment
                                    0x02,               // Length
                                    )
                                IO (Decode16,
                                    0x0CD8,             // Range Minimum
                                    0x0CD8,             // Range Maximum
                                    0x00,               // Alignment
                                    0x08,               // Length
                                    )
                                IO (Decode16,
                                    0x0000,             // Range Minimum
                                    0x0000,             // Range Maximum
                                    0x00,               // Alignment
                                    0x00,               // Length
                                    _Y11)
                                IO (Decode16,
                                    0x0000,             // Range Minimum
                                    0x0000,             // Range Maximum
                                    0x00,               // Alignment
                                    0x00,               // Length
                                    _Y13)
                                IO (Decode16,
                                    0x0000,             // Range Minimum
                                    0x0000,             // Range Maximum
                                    0x00,               // Alignment
                                    0x00,               // Length
                                    _Y12)
                                IO (Decode16,
                                    0x0900,             // Range Minimum
                                    0x0900,             // Range Maximum
                                    0x00,               // Alignment
                                    0x10,               // Length
                                    )
                                IO (Decode16,
                                    0x0910,             // Range Minimum
                                    0x0910,             // Range Maximum
                                    0x00,               // Alignment
                                    0x10,               // Length
                                    )
                                IO (Decode16,
                                    0x0060,             // Range Minimum
                                    0x0060,             // Range Maximum
                                    0x00,               // Alignment
                                    0x00,               // Length
                                    )
                                IO (Decode16,
                                    0x0064,             // Range Minimum
                                    0x0064,             // Range Maximum
                                    0x00,               // Alignment
                                    0x00,               // Length
                                    )
                                Memory32Fixed (ReadWrite,
                                    0x00000000,         // Address Base
                                    0x00000000,         // Address Length
                                    _Y14)
                                Memory32Fixed (ReadWrite,
                                    0xFEC01000,         // Address Base
                                    0x00001000,         // Address Length
                                    )
                                Memory32Fixed (ReadWrite,
                                    0xFEDC0000,         // Address Base
                                    0x00001000,         // Address Length
                                    )
                                Memory32Fixed (ReadWrite,
                                    0xFEE00000,         // Address Base
                                    0x00001000,         // Address Length
                                    )
                                Memory32Fixed (ReadWrite,
                                    0xFED80000,         // Address Base
                                    0x00010000,         // Address Length
                                    )
                                Memory32Fixed (ReadWrite,
                                    0x00000000,         // Address Base
                                    0x00000000,         // Address Length
                                    _Y15)
                                Memory32Fixed (ReadWrite,
                                    0x00000000,         // Address Base
                                    0x00000000,         // Address Length
                                    _Y16)
                                Memory32Fixed (ReadWrite,
                                    0x00000000,         // Address Base
                                    0x00000000,         // Address Length
                                    _Y17)
                            })
                            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                            {
                                CreateWordField (CRS, \_SB.PCI0.S900._Y11._MIN, PBB)  // _MIN: Minimum Base Address
                                CreateWordField (CRS, \_SB.PCI0.S900._Y11._MAX, PBH)  // _MAX: Maximum Base Address
                                CreateByteField (CRS, \_SB.PCI0.S900._Y11._LEN, PML)  // _LEN: Length
                                Store (PMBS, PBB) /* \_SB_.PCI0.S900._CRS.PBB_ */
                                Store (PMBS, PBH) /* \_SB_.PCI0.S900._CRS.PBH_ */
                                Store (PMLN, PML) /* \_SB_.PCI0.S900._CRS.PML_ */
                                If (SMBB)
                                {
                                    CreateWordField (CRS, \_SB.PCI0.S900._Y12._MIN, SMB1)  // _MIN: Minimum Base Address
                                    CreateWordField (CRS, \_SB.PCI0.S900._Y12._MAX, SMH1)  // _MAX: Maximum Base Address
                                    CreateByteField (CRS, \_SB.PCI0.S900._Y12._LEN, SML1)  // _LEN: Length
                                    Store (SMBB, SMB1) /* \_SB_.PCI0.S900._CRS.SMB1 */
                                    Store (SMBB, SMH1) /* \_SB_.PCI0.S900._CRS.SMH1 */
                                    Store (SMBL, SML1) /* \_SB_.PCI0.S900._CRS.SML1 */
                                    CreateWordField (CRS, \_SB.PCI0.S900._Y13._MIN, SMBZ)  // _MIN: Minimum Base Address
                                    CreateWordField (CRS, \_SB.PCI0.S900._Y13._MAX, SMH0)  // _MAX: Maximum Base Address
                                    CreateByteField (CRS, \_SB.PCI0.S900._Y13._LEN, SML0)  // _LEN: Length
                                    Store (SMB0, SMBZ) /* \_SB_.PCI0.S900._CRS.SMBZ */
                                    Store (SMB0, SMH0) /* \_SB_.PCI0.S900._CRS.SMH0 */
                                    Store (SMBM, SML0) /* \_SB_.PCI0.S900._CRS.SML0 */
                                }

                                If (APCB)
                                {
                                    CreateDWordField (CRS, \_SB.PCI0.S900._Y14._BAS, APB)  // _BAS: Base Address
                                    CreateDWordField (CRS, \_SB.PCI0.S900._Y14._LEN, APL)  // _LEN: Length
                                    Store (APCB, APB) /* \_SB_.PCI0.S900._CRS.APB_ */
                                    Store (APCL, APL) /* \_SB_.PCI0.S900._CRS.APL_ */
                                }

                                CreateDWordField (CRS, \_SB.PCI0.S900._Y15._BAS, SPIB)  // _BAS: Base Address
                                CreateDWordField (CRS, \_SB.PCI0.S900._Y15._LEN, SPIL)  // _LEN: Length
                                Store (0xFEC10000, SPIB) /* \_SB_.PCI0.S900._CRS.SPIB */
                                Store (0x1000, SPIL) /* \_SB_.PCI0.S900._CRS.SPIL */
                                If (WDTB)
                                {
                                    CreateDWordField (CRS, \_SB.PCI0.S900._Y16._BAS, WDTB)  // _BAS: Base Address
                                    CreateDWordField (CRS, \_SB.PCI0.S900._Y16._LEN, WDTL)  // _LEN: Length
                                    Store (\WDTB, WDTB) /* \_SB_.PCI0.S900._CRS.WDTB */
                                    Store (\WDTL, WDTL) /* \_SB_.PCI0.S900._CRS.WDTL */
                                }

                                CreateDWordField (CRS, \_SB.PCI0.S900._Y17._BAS, ROMB)  // _BAS: Base Address
                                CreateDWordField (CRS, \_SB.PCI0.S900._Y17._LEN, ROML)  // _LEN: Length
                                Store (0xFF000000, ROMB) /* \_SB_.PCI0.S900._CRS.ROMB */
                                Store (0x01000000, ROML) /* \_SB_.PCI0.S900._CRS.ROML */
                                Return (CRS) /* \_SB_.PCI0.S900.CRS_ */
                            }
                        }
                    }
                }

                Scope (\_SB)
                {
                    Scope (PCI0)
                    {
                        Scope (SBRG)
                        {
                            Method (RRIO, 4, NotSerialized)
                            {
                                Store ("RRIO", Debug)
                            }

                            Method (RDMA, 3, NotSerialized)
                            {
                                Store ("rDMA", Debug)
                            }
                        }
                    }
                }
            }

            Device (D01F)
            {
                Name (_ADR, 0x00140006)  // _ADR: Address
            }

            Device (GPP4)
            {
                Name (_ADR, 0x00020002)  // _ADR: Address
                Method (MPRW, 0, NotSerialized)
                {
                    Return (GPRW (0x0E, Zero))
                }

                Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
                {
                    If (PICM)
                    {
                        Return (AR20) /* \_SB_.AR20 */
                    }

                    Return (PD20) /* \_SB_.PD20 */
                }

                Device (WLAN)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                }
            }
        }
    }

    Scope (_GPE)
    {
        Method (_L08, 0, NotSerialized)  // _Lxx: Level-Triggered GPE, xx=0x00-0xFF
        {
            Notify (\_SB.PCI0.GPP0, 0x02) // Device Wake
            Notify (\_SB.PCI0.GPP1, 0x02) // Device Wake
            Notify (\_SB.PCI0.GPP5, 0x02) // Device Wake
            Notify (\_SB.PCI0.GP18, 0x02) // Device Wake
            Notify (\_SB.PWRB, 0x02) // Device Wake
        }

        Method (_L0D, 0, NotSerialized)  // _Lxx: Level-Triggered GPE, xx=0x00-0xFF
        {
            Notify (\_SB.PCI0.GPP2, 0x02) // Device Wake
            Notify (\_SB.PWRB, 0x02) // Device Wake
        }

        Method (_L0F, 0, NotSerialized)  // _Lxx: Level-Triggered GPE, xx=0x00-0xFF
        {
            Notify (\_SB.PCI0.GPP3, 0x02) // Device Wake
            Notify (\_SB.PWRB, 0x02) // Device Wake
        }

        Method (_L19, 0, NotSerialized)  // _Lxx: Level-Triggered GPE, xx=0x00-0xFF
        {
            Notify (\_SB.PCI0.GP17, 0x02) // Device Wake
            Notify (\_SB.PCI0.GP17.XHC0, 0x02) // Device Wake
            Notify (\_SB.PCI0.GP17.XHC1, 0x02) // Device Wake
            Notify (\_SB.PWRB, 0x02) // Device Wake
        }
    }

    Scope (_SB)
    {
        Device (PWRB)
        {
            Name (_HID, EisaId ("PNP0C0C") /* Power Button Device */)  // _HID: Hardware ID
            Name (_UID, 0xAA)  // _UID: Unique ID
            Name (_STA, 0x0B)  // _STA: Status
        }
    }

    Name (_S0, Package (0x04)  // _S0_: S0 System State
    {
        Zero, 
        Zero, 
        Zero, 
        Zero
    })
    Name (_S3, Package (0x04)  // _S3_: S3 System State
    {
        0x03, 
        Zero, 
        Zero, 
        Zero
    })
    Name (_S4, Package (0x04)  // _S4_: S4 System State
    {
        0x04, 
        Zero, 
        Zero, 
        Zero
    })
    Name (_S5, Package (0x04)  // _S5_: S5 System State
    {
        0x05, 
        Zero, 
        Zero, 
        Zero
    })
    Method (_PTS, 1, NotSerialized)  // _PTS: Prepare To Sleep
    {
        If (Arg0)
        {
            \_SB.PCI0.SBRG.EC0.EC0S (Arg0)
            SBDS (Arg0)
            \_SB.TPM.TPTS (Arg0)
            SPTS (Arg0)
            \_SB.PCI0.NPTS (Arg0)
        }
    }

    Method (_WAK, 1, NotSerialized)  // _WAK: Wake
    {
        ShiftLeft (Arg0, 0x04, DBG8) /* \DBG8 */
        \_SB.PCI0.NWAK (Arg0)
        If (LOr (LEqual (Arg0, 0x03), LEqual (Arg0, 0x04)))
        {
            If (LNotEqual (PICM, Zero))
            {
                \_SB.PCI0.NAPE ()
            }
        }

        ShiftLeft (Arg0, 0x04, DBG8) /* \DBG8 */
        SWAK (Arg0)
        SBDW (Arg0)
        \_SB.PCI0.SBRG.EC0.EC0W (Arg0)
        Return (WAKP) /* \WAKP */
    }

    Scope (_SB)
    {
        Device (PLTF)
        {
            Name (_HID, "ACPI0010" /* Processor Container Device */)  // _HID: Hardware ID
            Name (_CID, EisaId ("PNP0A05") /* Generic Container Device */)  // _CID: Compatible ID
            Name (_UID, One)  // _UID: Unique ID
            Device (P000)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, One)  // _UID: Unique ID
            }

            Device (P001)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x02)  // _UID: Unique ID
            }

            Device (P002)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x03)  // _UID: Unique ID
            }

            Device (P003)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x04)  // _UID: Unique ID
            }

            Device (P004)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x05)  // _UID: Unique ID
            }

            Device (P005)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x06)  // _UID: Unique ID
            }

            Device (P006)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x07)  // _UID: Unique ID
            }

            Device (P007)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x08)  // _UID: Unique ID
            }

            Device (P008)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x09)  // _UID: Unique ID
            }

            Device (P009)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x0A)  // _UID: Unique ID
            }

            Device (P00A)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x0B)  // _UID: Unique ID
            }

            Device (P00B)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x0C)  // _UID: Unique ID
            }

            Device (P00C)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x0D)  // _UID: Unique ID
            }

            Device (P00D)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x0E)  // _UID: Unique ID
            }

            Device (P00E)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x0F)  // _UID: Unique ID
            }

            Device (P00F)
            {
                Name (_HID, "ACPI0007" /* Processor Device */)  // _HID: Hardware ID
                Name (_UID, 0x10)  // _UID: Unique ID
            }
        }
    }

    Scope (_SB)
    {
        OperationRegion (PIRQ, SystemIO, 0x0C00, 0x02)
        Field (PIRQ, ByteAcc, NoLock, Preserve)
        {
            PIDX,   8, 
            PDAT,   8
        }

        IndexField (PIDX, PDAT, ByteAcc, NoLock, Preserve)
        {
            PIRA,   8, 
            PIRB,   8, 
            PIRC,   8, 
            PIRD,   8, 
            PIRE,   8, 
            PIRF,   8, 
            PIRG,   8, 
            PIRH,   8, 
            Offset (0x0C), 
            SIRA,   8, 
            SIRB,   8, 
            SIRC,   8, 
            SIRD,   8, 
            PIRS,   8, 
            Offset (0x13), 
            HDAD,   8, 
            Offset (0x17), 
            SDCL,   8, 
            Offset (0x1A), 
            SDIO,   8, 
            Offset (0x30), 
            USB1,   8, 
            Offset (0x34), 
            USB3,   8, 
            Offset (0x41), 
            SATA,   8, 
            Offset (0x62), 
            GIOC,   8, 
            Offset (0x70), 
            I2C0,   8, 
            I2C1,   8, 
            I2C2,   8, 
            I2C3,   8, 
            URT0,   8, 
            URT1,   8, 
            Offset (0x80), 
            AIRA,   8, 
            AIRB,   8, 
            AIRC,   8, 
            AIRD,   8, 
            AIRE,   8, 
            AIRF,   8, 
            AIRG,   8, 
            AIRH,   8
        }

        OperationRegion (KBDD, SystemIO, 0x64, One)
        Field (KBDD, ByteAcc, NoLock, Preserve)
        {
            PD64,   8
        }

        Method (DSPI, 0, NotSerialized)
        {
            INTA (0x1F)
            INTB (0x1F)
            INTC (0x1F)
            INTD (0x1F)
            Store (PD64, Local1)
            Store (0x1F, PIRE) /* \_SB_.PIRE */
            Store (0x1F, PIRF) /* \_SB_.PIRF */
            Store (0x1F, PIRG) /* \_SB_.PIRG */
            Store (0x1F, PIRH) /* \_SB_.PIRH */
            Store (PD64, Local1)
            Store (0x10, AIRA) /* \_SB_.AIRA */
            Store (0x11, AIRB) /* \_SB_.AIRB */
            Store (0x12, AIRC) /* \_SB_.AIRC */
            Store (0x13, AIRD) /* \_SB_.AIRD */
            Store (0x14, AIRE) /* \_SB_.AIRE */
            Store (0x15, AIRF) /* \_SB_.AIRF */
            Store (0x16, AIRG) /* \_SB_.AIRG */
            Store (0x17, AIRH) /* \_SB_.AIRH */
        }

        Method (INTA, 1, NotSerialized)
        {
            Store (Arg0, PIRA) /* \_SB_.PIRA */
            Store (Arg0, HDAD) /* \_SB_.HDAD */
        }

        Method (INTB, 1, NotSerialized)
        {
            Store (Arg0, PIRB) /* \_SB_.PIRB */
        }

        Method (INTC, 1, NotSerialized)
        {
            Store (Arg0, PIRC) /* \_SB_.PIRC */
            Store (Arg0, USB1) /* \_SB_.USB1 */
            Store (Arg0, USB3) /* \_SB_.USB3 */
        }

        Method (INTD, 1, NotSerialized)
        {
            Store (Arg0, PIRD) /* \_SB_.PIRD */
            Store (Arg0, SATA) /* \_SB_.SATA */
        }

        Name (BUFA, ResourceTemplate ()
        {
            IRQ (Level, ActiveLow, Shared, )
                {15}
        })
        Name (IPRA, ResourceTemplate ()
        {
            IRQ (Level, ActiveLow, Shared, )
                {5,10,11}
        })
        Name (IPRB, ResourceTemplate ()
        {
            IRQ (Level, ActiveLow, Shared, )
                {5,10,11}
        })
        Name (IPRC, ResourceTemplate ()
        {
            IRQ (Level, ActiveLow, Shared, )
                {5,10,11}
        })
        Name (IPRD, ResourceTemplate ()
        {
            IRQ (Level, ActiveLow, Shared, )
                {5,10,11}
        })
        Device (LNKA)
        {
            Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
            Name (_UID, One)  // _UID: Unique ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (PIRA)
                {
                    Return (0x0B)
                }
                Else
                {
                    Return (0x09)
                }
            }

            Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
            {
                Return (PRSA) /* \_SB_.PRSA */
            }

            Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
            {
                INTA (0x1F)
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                CreateWordField (BUFA, One, IRQX)
                ShiftLeft (One, PIRA, IRQX) /* \_SB_.LNKA._CRS.IRQX */
                Return (BUFA) /* \_SB_.BUFA */
            }

            Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
            {
                CreateWordField (Arg0, One, IRA)
                FindSetRightBit (IRA, Local0)
                Decrement (Local0)
                INTA (Local0)
            }
        }

        Device (LNKB)
        {
            Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
            Name (_UID, 0x02)  // _UID: Unique ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (PIRB)
                {
                    Return (0x0B)
                }
                Else
                {
                    Return (0x09)
                }
            }

            Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
            {
                Return (PRSB) /* \_SB_.PRSB */
            }

            Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
            {
                INTB (0x1F)
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                CreateWordField (BUFA, One, IRQX)
                ShiftLeft (One, PIRB, IRQX) /* \_SB_.LNKB._CRS.IRQX */
                Return (BUFA) /* \_SB_.BUFA */
            }

            Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
            {
                CreateWordField (Arg0, One, IRA)
                FindSetRightBit (IRA, Local0)
                Decrement (Local0)
                INTB (Local0)
            }
        }

        Device (LNKC)
        {
            Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
            Name (_UID, 0x03)  // _UID: Unique ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (PIRC)
                {
                    Return (0x0B)
                }
                Else
                {
                    Return (0x09)
                }
            }

            Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
            {
                Return (PRSC) /* \_SB_.PRSC */
            }

            Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
            {
                INTC (0x1F)
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                CreateWordField (BUFA, One, IRQX)
                ShiftLeft (One, PIRC, IRQX) /* \_SB_.LNKC._CRS.IRQX */
                Return (BUFA) /* \_SB_.BUFA */
            }

            Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
            {
                CreateWordField (Arg0, One, IRA)
                FindSetRightBit (IRA, Local0)
                Decrement (Local0)
                INTC (Local0)
            }
        }

        Device (LNKD)
        {
            Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
            Name (_UID, 0x04)  // _UID: Unique ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (PIRD)
                {
                    Return (0x0B)
                }
                Else
                {
                    Return (0x09)
                }
            }

            Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
            {
                Return (PRSD) /* \_SB_.PRSD */
            }

            Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
            {
                INTD (0x1F)
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                CreateWordField (BUFA, One, IRQX)
                ShiftLeft (One, PIRD, IRQX) /* \_SB_.LNKD._CRS.IRQX */
                Return (BUFA) /* \_SB_.BUFA */
            }

            Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
            {
                CreateWordField (Arg0, One, IRA)
                FindSetRightBit (IRA, Local0)
                Decrement (Local0)
                INTD (Local0)
            }
        }

        Device (LNKE)
        {
            Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
            Name (_UID, 0x05)  // _UID: Unique ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (PIRE)
                {
                    Return (0x0B)
                }
                Else
                {
                    Return (0x09)
                }
            }

            Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
            {
                Return (PRSE) /* \_SB_.PRSE */
            }

            Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
            {
                Store (0x1F, PIRE) /* \_SB_.PIRE */
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                CreateWordField (BUFA, One, IRQX)
                ShiftLeft (One, PIRE, IRQX) /* \_SB_.LNKE._CRS.IRQX */
                Return (BUFA) /* \_SB_.BUFA */
            }

            Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
            {
                CreateWordField (Arg0, One, IRA)
                FindSetRightBit (PIRE, Local0)
                Decrement (Local0)
                Store (Local0, PIRE) /* \_SB_.PIRE */
            }
        }

        Device (LNKF)
        {
            Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
            Name (_UID, 0x06)  // _UID: Unique ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (PIRF)
                {
                    Return (0x0B)
                }
                Else
                {
                    Return (0x09)
                }
            }

            Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
            {
                Return (PRSF) /* \_SB_.PRSF */
            }

            Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
            {
                Store (0x1F, PIRF) /* \_SB_.PIRF */
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                CreateWordField (BUFA, One, IRQX)
                ShiftLeft (One, PIRF, IRQX) /* \_SB_.LNKF._CRS.IRQX */
                Return (BUFA) /* \_SB_.BUFA */
            }

            Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
            {
                CreateWordField (Arg0, One, IRA)
                FindSetRightBit (IRA, Local0)
                Decrement (Local0)
                Store (Local0, PIRF) /* \_SB_.PIRF */
            }
        }

        Device (LNKG)
        {
            Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
            Name (_UID, 0x07)  // _UID: Unique ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (PIRG)
                {
                    Return (0x0B)
                }
                Else
                {
                    Return (0x09)
                }
            }

            Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
            {
                Return (PRSG) /* \_SB_.PRSG */
            }

            Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
            {
                Store (0x1F, PIRG) /* \_SB_.PIRG */
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                CreateWordField (BUFA, One, IRQX)
                ShiftLeft (One, PIRG, IRQX) /* \_SB_.LNKG._CRS.IRQX */
                Return (BUFA) /* \_SB_.BUFA */
            }

            Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
            {
                CreateWordField (Arg0, One, IRA)
                FindSetRightBit (IRA, Local0)
                Decrement (Local0)
                Store (Local0, PIRG) /* \_SB_.PIRG */
            }
        }

        Device (LNKH)
        {
            Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
            Name (_UID, 0x08)  // _UID: Unique ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (PIRH)
                {
                    Return (0x0B)
                }
                Else
                {
                    Return (0x09)
                }
            }

            Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
            {
                Return (PRSH) /* \_SB_.PRSH */
            }

            Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
            {
                Store (0x1F, PIRH) /* \_SB_.PIRH */
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                CreateWordField (BUFA, One, IRQX)
                ShiftLeft (One, PIRH, IRQX) /* \_SB_.LNKH._CRS.IRQX */
                Return (BUFA) /* \_SB_.BUFA */
            }

            Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
            {
                CreateWordField (Arg0, One, IRA)
                FindSetRightBit (IRA, Local0)
                Decrement (Local0)
                Store (Local0, PIRH) /* \_SB_.PIRH */
            }
        }
    }

    Name (OSTB, Ones)
    Name (TPOS, Zero)
    Name (LINX, Zero)
    Name (OSSP, Zero)
    Method (SEQL, 2, Serialized)
    {
        Store (SizeOf (Arg0), Local0)
        Store (SizeOf (Arg1), Local1)
        If (LNotEqual (Local0, Local1))
        {
            Return (Zero)
        }

        Name (BUF0, Buffer (Local0){})
        Store (Arg0, BUF0) /* \SEQL.BUF0 */
        Name (BUF1, Buffer (Local0){})
        Store (Arg1, BUF1) /* \SEQL.BUF1 */
        Store (Zero, Local2)
        While (LLess (Local2, Local0))
        {
            Store (DerefOf (Index (BUF0, Local2)), Local3)
            Store (DerefOf (Index (BUF1, Local2)), Local4)
            If (LNotEqual (Local3, Local4))
            {
                Return (Zero)
            }

            Increment (Local2)
        }

        Return (One)
    }

    Method (OSTP, 0, NotSerialized)
    {
        If (LEqual (OSTB, Ones))
        {
            If (CondRefOf (\_OSI, Local0))
            {
                Store (Zero, OSTB) /* \OSTB */
                Store (Zero, TPOS) /* \TPOS */
                If (_OSI ("Windows 2001"))
                {
                    Store (0x08, OSTB) /* \OSTB */
                    Store (0x08, TPOS) /* \TPOS */
                }

                If (_OSI ("Windows 2001.1"))
                {
                    Store (0x20, OSTB) /* \OSTB */
                    Store (0x20, TPOS) /* \TPOS */
                }

                If (_OSI ("Windows 2001 SP1"))
                {
                    Store (0x10, OSTB) /* \OSTB */
                    Store (0x10, TPOS) /* \TPOS */
                }

                If (_OSI ("Windows 2001 SP2"))
                {
                    Store (0x11, OSTB) /* \OSTB */
                    Store (0x11, TPOS) /* \TPOS */
                }

                If (_OSI ("Windows 2001 SP3"))
                {
                    Store (0x12, OSTB) /* \OSTB */
                    Store (0x12, TPOS) /* \TPOS */
                }

                If (_OSI ("Windows 2006"))
                {
                    Store (0x40, OSTB) /* \OSTB */
                    Store (0x40, TPOS) /* \TPOS */
                }

                If (_OSI ("Windows 2006 SP1"))
                {
                    Store (0x41, OSTB) /* \OSTB */
                    Store (0x41, TPOS) /* \TPOS */
                    Store (One, OSSP) /* \OSSP */
                }

                If (_OSI ("Windows 2009"))
                {
                    Store (One, OSSP) /* \OSSP */
                    Store (0x50, OSTB) /* \OSTB */
                    Store (0x50, TPOS) /* \TPOS */
                }

                If (_OSI ("Windows 2012"))
                {
                    Store (One, OSSP) /* \OSSP */
                    Store (0x60, OSTB) /* \OSTB */
                    Store (0x60, TPOS) /* \TPOS */
                }

                If (_OSI ("Windows 2013"))
                {
                    Store (One, OSSP) /* \OSSP */
                    Store (0x61, OSTB) /* \OSTB */
                    Store (0x61, TPOS) /* \TPOS */
                }

                If (_OSI ("Windows 2015"))
                {
                    Store (One, OSSP) /* \OSSP */
                    Store (0x70, OSTB) /* \OSTB */
                    Store (0x70, TPOS) /* \TPOS */
                }

                If (_OSI ("Linux"))
                {
                    Store (One, LINX) /* \LINX */
                    Store (0x80, OSTB) /* \OSTB */
                    Store (0x80, TPOS) /* \TPOS */
                }
            }
            ElseIf (CondRefOf (\_OS, Local0))
            {
                If (SEQL (_OS, "Microsoft Windows"))
                {
                    Store (One, OSTB) /* \OSTB */
                    Store (One, TPOS) /* \TPOS */
                }
                ElseIf (SEQL (_OS, "Microsoft WindowsME: Millennium Edition"))
                {
                    Store (0x02, OSTB) /* \OSTB */
                    Store (0x02, TPOS) /* \TPOS */
                }
                ElseIf (SEQL (_OS, "Microsoft Windows NT"))
                {
                    Store (0x04, OSTB) /* \OSTB */
                    Store (0x04, TPOS) /* \TPOS */
                }
                Else
                {
                    Store (Zero, OSTB) /* \OSTB */
                    Store (Zero, TPOS) /* \TPOS */
                }
            }
            Else
            {
                Store (Zero, OSTB) /* \OSTB */
                Store (Zero, TPOS) /* \TPOS */
            }
        }

        Return (OSTB) /* \OSTB */
    }

    Scope (_SB.PCI0)
    {
        Method (_INI, 0, NotSerialized)  // _INI: Initialize
        {
            If (LNotEqual (PICM, Zero))
            {
                DSPI ()
                NAPE ()
            }

            OSTP ()
            OSFL ()
        }
    }

    Name (TSOS, 0x75)
    If (CondRefOf (\_OSI))
    {
        If (_OSI ("Windows 2009"))
        {
            Store (0x50, TSOS) /* \TSOS */
        }

        If (_OSI ("Windows 2015"))
        {
            Store (0x70, TSOS) /* \TSOS */
        }
    }

    Scope (_SB)
    {
        Name (D0S0, One)
        Name (D3S0, One)
        Name (D0S1, One)
        Name (D3S1, One)
        PowerResource (P0S0, 0x00, 0x0000)
        {
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (D0S0) /* \_SB_.D0S0 */
            }

            Method (_ON, 0, NotSerialized)  // _ON_: Power On
            {
                Store (One, D0S0) /* \_SB_.D0S0 */
            }

            Method (_OFF, 0, NotSerialized)  // _OFF: Power Off
            {
                Store (Zero, D0S0) /* \_SB_.D0S0 */
            }
        }

        PowerResource (P3S0, 0x00, 0x0000)
        {
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (D3S0) /* \_SB_.D3S0 */
            }

            Method (_ON, 0, NotSerialized)  // _ON_: Power On
            {
                Store (One, D3S0) /* \_SB_.D3S0 */
            }

            Method (_OFF, 0, NotSerialized)  // _OFF: Power Off
            {
                Store (Zero, D3S0) /* \_SB_.D3S0 */
            }
        }

        PowerResource (P0S1, 0x00, 0x0000)
        {
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (D0S1) /* \_SB_.D0S1 */
            }

            Method (_ON, 0, NotSerialized)  // _ON_: Power On
            {
                Store (One, D0S1) /* \_SB_.D0S1 */
            }

            Method (_OFF, 0, NotSerialized)  // _OFF: Power Off
            {
                Store (Zero, D0S1) /* \_SB_.D0S1 */
            }
        }

        PowerResource (P3S1, 0x00, 0x0000)
        {
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (D3S1) /* \_SB_.D3S1 */
            }

            Method (_ON, 0, NotSerialized)  // _ON_: Power On
            {
                Store (One, D3S1) /* \_SB_.D3S1 */
            }

            Method (_OFF, 0, NotSerialized)  // _OFF: Power Off
            {
                Store (Zero, D3S1) /* \_SB_.D3S1 */
            }
        }

        OperationRegion (SMIC, SystemMemory, 0xFED80000, 0x00800000)
        Field (SMIC, ByteAcc, NoLock, Preserve)
        {
            Offset (0x36A), 
            SMIB,   8
        }

        OperationRegion (SSMI, SystemIO, SMIB, 0x02)
        Field (SSMI, AnyAcc, NoLock, Preserve)
        {
            SMIW,   16
        }

        OperationRegion (ECMC, SystemIO, 0x72, 0x02)
        Field (ECMC, AnyAcc, NoLock, Preserve)
        {
            ECMI,   8, 
            ECMD,   8
        }

        IndexField (ECMI, ECMD, ByteAcc, NoLock, Preserve)
        {
            Offset (0x08), 
            FRTB,   32
        }

        OperationRegion (FRTP, SystemMemory, FRTB, 0x0100)
        Field (FRTP, AnyAcc, NoLock, Preserve)
        {
            PEBA,   32, 
                ,   5, 
            IC0E,   1, 
            IC1E,   1, 
            IC2E,   1, 
            IC3E,   1, 
            IC4E,   1, 
            IC5E,   1, 
            UT0E,   1, 
            UT1E,   1, 
                ,   1, 
                ,   1, 
            ST_E,   1, 
            UT2E,   1, 
                ,   1, 
            EMMD,   2, 
                ,   3, 
            XHCE,   1, 
                ,   1, 
                ,   1, 
            UT3E,   1, 
            ESPI,   1, 
            EMME,   1, 
            HFPE,   1, 
            Offset (0x08), 
            PCEF,   1, 
                ,   4, 
            IC0D,   1, 
            IC1D,   1, 
            IC2D,   1, 
            IC3D,   1, 
            IC4D,   1, 
            IC5D,   1, 
            UT0D,   1, 
            UT1D,   1, 
                ,   1, 
                ,   1, 
            ST_D,   1, 
            UT2D,   1, 
                ,   1, 
            EHCD,   1, 
                ,   4, 
            XHCD,   1, 
            SD_D,   1, 
                ,   1, 
            UT3D,   1, 
                ,   1, 
            EMD3,   1, 
                ,   2, 
            S03D,   1, 
            FW00,   16, 
            FW01,   32, 
            FW02,   16, 
            FW03,   32, 
            SDS0,   8, 
            SDS1,   8, 
            CZFG,   1, 
            Offset (0x20), 
            SD10,   32, 
            EH10,   32, 
            XH10,   32, 
            STBA,   32
        }

        OperationRegion (FCFG, SystemMemory, PEBA, 0x01000000)
        Field (FCFG, DWordAcc, NoLock, Preserve)
        {
            Offset (0xA3044), 
            IPDE,   32, 
            IMPE,   32, 
            Offset (0xA3078), 
                ,   2, 
            LDQ0,   1, 
            Offset (0xA30CB), 
                ,   7, 
            AUSS,   1
        }

        OperationRegion (IOMX, SystemMemory, 0xFED80D00, 0x0100)
        Field (IOMX, AnyAcc, NoLock, Preserve)
        {
            Offset (0x15), 
            IM15,   8, 
            IM16,   8, 
            Offset (0x1F), 
            IM1F,   8, 
            IM20,   8, 
            Offset (0x44), 
            IM44,   8, 
            Offset (0x46), 
            IM46,   8, 
            Offset (0x4A), 
            IM4A,   8, 
            IM4B,   8, 
            Offset (0x57), 
            IM57,   8, 
            IM58,   8, 
            Offset (0x68), 
            IM68,   8, 
            IM69,   8, 
            IM6A,   8, 
            IM6B,   8, 
            Offset (0x6D), 
            IM6D,   8
        }

        OperationRegion (FACR, SystemMemory, 0xFED81E00, 0x0100)
        Field (FACR, AnyAcc, NoLock, Preserve)
        {
            Offset (0x80), 
                ,   28, 
            RD28,   1, 
                ,   1, 
            RQTY,   1, 
            Offset (0x84), 
                ,   28, 
            SD28,   1, 
                ,   1, 
            Offset (0xA0), 
            PG1A,   1
        }

        OperationRegion (EMMX, SystemMemory, 0xFEDD5800, 0x0130)
        Field (EMMX, AnyAcc, NoLock, Preserve)
        {
            Offset (0xD0), 
                ,   17, 
            FC18,   1, 
            FC33,   1, 
                ,   7, 
            CD_T,   1, 
            WP_T,   1
        }

        OperationRegion (EMMB, SystemMemory, 0xFEDD5800, 0x0130)
        Field (EMMB, AnyAcc, NoLock, Preserve)
        {
            Offset (0xA4), 
            E0A4,   32, 
            E0A8,   32, 
            Offset (0xB0), 
            E0B0,   32, 
            Offset (0xD0), 
            E0D0,   32, 
            Offset (0x116), 
            E116,   32
        }

        Name (SVBF, Buffer (0x0100)
        {
             0x00                                             // .
        })
        CreateDWordField (SVBF, Zero, S0A4)
        CreateDWordField (SVBF, 0x04, S0A8)
        CreateDWordField (SVBF, 0x08, S0B0)
        CreateDWordField (SVBF, 0x0C, S0D0)
        CreateDWordField (SVBF, 0x10, S116)
        Method (SECR, 0, Serialized)
        {
            Store (E116, S116) /* \_SB_.S116 */
            Store (Zero, RQTY) /* \_SB_.RQTY */
            Store (One, RD28) /* \_SB_.RD28 */
            Store (SD28, Local0)
            While (Local0)
            {
                Store (SD28, Local0)
            }
        }

        Method (RECR, 0, Serialized)
        {
            Store (S116, E116) /* \_SB_.E116 */
        }

        OperationRegion (LUIE, SystemMemory, 0xFEDC0020, 0x04)
        Field (LUIE, AnyAcc, NoLock, Preserve)
        {
            IER0,   1, 
            IER1,   1, 
            IER2,   1, 
            IER3,   1, 
            UOL0,   1, 
            UOL1,   1, 
            UOL2,   1, 
            UOL3,   1, 
            WUR0,   2, 
            WUR1,   2, 
            WUR2,   2, 
            WUR3,   2
        }

        Method (FRUI, 2, Serialized)
        {
            If (LEqual (Arg0, Zero))
            {
                Store (IUA0, Arg1)
            }

            If (LEqual (Arg0, One))
            {
                Store (IUA1, Arg1)
            }

            If (LEqual (Arg0, 0x02))
            {
                Store (IUA2, Arg1)
            }

            If (LEqual (Arg0, 0x03))
            {
                Store (IUA3, Arg1)
            }
        }

        Method (SRAD, 2, Serialized)
        {
            ShiftLeft (Arg0, One, Local0)
            Add (Local0, 0xFED81E40, Local0)
            OperationRegion (ADCR, SystemMemory, Local0, 0x02)
            Field (ADCR, ByteAcc, NoLock, Preserve)
            {
                ADTD,   2, 
                ADPS,   1, 
                ADPD,   1, 
                ADSO,   1, 
                ADSC,   1, 
                ADSR,   1, 
                ADIS,   1, 
                ADDS,   3
            }

            Store (One, ADIS) /* \_SB_.SRAD.ADIS */
            Store (Zero, ADSR) /* \_SB_.SRAD.ADSR */
            Stall (Arg1)
            Store (One, ADSR) /* \_SB_.SRAD.ADSR */
            Store (Zero, ADIS) /* \_SB_.SRAD.ADIS */
            Stall (Arg1)
        }

        Method (DSAD, 2, Serialized)
        {
            ShiftLeft (Arg0, One, Local0)
            Add (Local0, 0xFED81E40, Local0)
            OperationRegion (ADCR, SystemMemory, Local0, 0x02)
            Field (ADCR, ByteAcc, NoLock, Preserve)
            {
                ADTD,   2, 
                ADPS,   1, 
                ADPD,   1, 
                ADSO,   1, 
                ADSC,   1, 
                ADSR,   1, 
                ADIS,   1, 
                ADDS,   3
            }

            If (LNotEqual (Arg0, ADTD))
            {
                If (LEqual (Arg1, Zero))
                {
                    Store (Zero, ADTD) /* \_SB_.DSAD.ADTD */
                    Store (One, ADPD) /* \_SB_.DSAD.ADPD */
                    Store (ADDS, Local0)
                    While (LNotEqual (Local0, 0x07))
                    {
                        Store (ADDS, Local0)
                    }
                }

                If (LEqual (Arg1, 0x03))
                {
                    Store (Zero, ADPD) /* \_SB_.DSAD.ADPD */
                    Store (ADDS, Local0)
                    While (LNotEqual (Local0, Zero))
                    {
                        Store (ADDS, Local0)
                    }

                    Store (0x03, ADTD) /* \_SB_.DSAD.ADTD */
                }
            }
        }

        Method (HSAD, 2, Serialized)
        {
            ShiftLeft (One, Arg0, Local3)
            ShiftLeft (Arg0, One, Local0)
            Add (Local0, 0xFED81E40, Local0)
            OperationRegion (ADCR, SystemMemory, Local0, 0x02)
            Field (ADCR, ByteAcc, NoLock, Preserve)
            {
                ADTD,   2, 
                ADPS,   1, 
                ADPD,   1, 
                ADSO,   1, 
                ADSC,   1, 
                ADSR,   1, 
                ADIS,   1, 
                ADDS,   3
            }

            If (LNotEqual (Arg1, ADTD))
            {
                If (LEqual (Arg1, Zero))
                {
                    Store (One, PG1A) /* \_SB_.PG1A */
                    Store (Zero, ADTD) /* \_SB_.HSAD.ADTD */
                    Store (One, ADPD) /* \_SB_.HSAD.ADPD */
                    Store (ADDS, Local0)
                    While (LNotEqual (Local0, 0x07))
                    {
                        Store (ADDS, Local0)
                    }

                    Store (One, RQTY) /* \_SB_.RQTY */
                    Store (One, RD28) /* \_SB_.RD28 */
                    Store (SD28, Local0)
                    While (LNot (Local0))
                    {
                        Store (SD28, Local0)
                    }
                }

                If (LEqual (Arg1, 0x03))
                {
                    Store (Zero, RQTY) /* \_SB_.RQTY */
                    Store (One, RD28) /* \_SB_.RD28 */
                    Store (SD28, Local0)
                    While (Local0)
                    {
                        Store (SD28, Local0)
                    }

                    Store (Zero, ADPD) /* \_SB_.HSAD.ADPD */
                    Store (ADDS, Local0)
                    While (LNotEqual (Local0, Zero))
                    {
                        Store (ADDS, Local0)
                    }

                    Store (0x03, ADTD) /* \_SB_.HSAD.ADTD */
                    Store (Zero, PG1A) /* \_SB_.PG1A */
                }
            }
        }

        OperationRegion (FPIC, SystemIO, 0x0C00, 0x02)
        Field (ECMC, AnyAcc, NoLock, Preserve)
        {
            FPII,   8, 
            FPID,   8
        }

        IndexField (FPII, FPID, ByteAcc, NoLock, Preserve)
        {
            Offset (0xF4), 
            IUA0,   8, 
            IUA1,   8, 
            Offset (0xF8), 
            IUA2,   8, 
            IUA3,   8
        }

        Device (HFP1)
        {
            Name (_HID, "AMDI0060")  // _HID: Hardware ID
            Name (_UID, Zero)  // _UID: Unique ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (HFPE)
                {
                    Return (0x0F)
                }
                Else
                {
                    Return (Zero)
                }
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                Name (RBUF, ResourceTemplate ()
                {
                    Memory32Fixed (ReadWrite,
                        0xFEC11000,         // Address Base
                        0x00000100,         // Address Length
                        )
                })
                Return (RBUF) /* \_SB_.HFP1._CRS.RBUF */
            }
        }

        Device (GPIO)
        {
            Name (_HID, "AMDI0030")  // _HID: Hardware ID
            Name (_CID, "AMDI0030")  // _CID: Compatible ID
            Name (_UID, Zero)  // _UID: Unique ID
            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                Name (RBUF, ResourceTemplate ()
                {
                    Interrupt (ResourceConsumer, Level, ActiveLow, Shared, ,, )
                    {
                        0x00000007,
                    }
                    Memory32Fixed (ReadWrite,
                        0xFED81500,         // Address Base
                        0x00000400,         // Address Length
                        )
                    Memory32Fixed (ReadWrite,
                        0xFED81200,         // Address Base
                        0x00000100,         // Address Length
                        )
                })
                Return (RBUF) /* \_SB_.GPIO._CRS.RBUF */
            }

            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (TSOS, 0x70))
                {
                    Return (0x0F)
                }
                Else
                {
                    Return (Zero)
                }
            }
        }

        Device (FUR0)
        {
            Name (_HID, "AMDI0020")  // _HID: Hardware ID
            Name (_UID, Zero)  // _UID: Unique ID
            Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
            {
                IRQ (Edge, ActiveHigh, Exclusive, )
                    {3}
                Memory32Fixed (ReadWrite,
                    0xFEDC9000,         // Address Base
                    0x00001000,         // Address Length
                    )
                Memory32Fixed (ReadWrite,
                    0xFEDC7000,         // Address Base
                    0x00001000,         // Address Length
                    )
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (TSOS, 0x70))
                {
                    If (LEqual (UT0E, One))
                    {
                        If (IER0)
                        {
                            Return (Zero)
                        }

                        Return (0x0F)
                    }

                    Return (Zero)
                }
                Else
                {
                    Return (Zero)
                }
            }
        }

        Device (FUR1)
        {
            Name (_HID, "AMDI0020")  // _HID: Hardware ID
            Name (_UID, One)  // _UID: Unique ID
            Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
            {
                IRQ (Edge, ActiveHigh, Exclusive, )
                    {4}
                Memory32Fixed (ReadWrite,
                    0xFEDCA000,         // Address Base
                    0x00001000,         // Address Length
                    )
                Memory32Fixed (ReadWrite,
                    0xFEDC8000,         // Address Base
                    0x00001000,         // Address Length
                    )
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (TSOS, 0x70))
                {
                    If (LEqual (UT1E, One))
                    {
                        If (UOL1)
                        {
                            Return (Zero)
                        }

                        Return (0x0F)
                    }

                    Return (Zero)
                }
                Else
                {
                    Return (Zero)
                }
            }
        }

        Device (FUR2)
        {
            Name (_HID, "AMDI0020")  // _HID: Hardware ID
            Name (_UID, 0x02)  // _UID: Unique ID
            Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
            {
                IRQ (Edge, ActiveHigh, Exclusive, )
                    {3}
                Memory32Fixed (ReadWrite,
                    0xFEDCE000,         // Address Base
                    0x00001000,         // Address Length
                    )
                Memory32Fixed (ReadWrite,
                    0xFEDCC000,         // Address Base
                    0x00001000,         // Address Length
                    )
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (TSOS, 0x70))
                {
                    If (LEqual (UT2E, One))
                    {
                        If (UOL2)
                        {
                            Return (Zero)
                        }

                        Return (0x0F)
                    }

                    Return (Zero)
                }
                Else
                {
                    Return (Zero)
                }
            }
        }

        Device (FUR3)
        {
            Name (_HID, "AMDI0020")  // _HID: Hardware ID
            Name (_UID, 0x03)  // _UID: Unique ID
            Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
            {
                IRQ (Edge, ActiveHigh, Exclusive, )
                    {4}
                Memory32Fixed (ReadWrite,
                    0xFEDCF000,         // Address Base
                    0x00001000,         // Address Length
                    )
                Memory32Fixed (ReadWrite,
                    0xFEDCD000,         // Address Base
                    0x00001000,         // Address Length
                    )
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (TSOS, 0x70))
                {
                    If (LEqual (UT3E, One))
                    {
                        If (UOL3)
                        {
                            Return (Zero)
                        }

                        Return (0x0F)
                    }

                    Return (Zero)
                }
                Else
                {
                    Return (Zero)
                }
            }
        }

        Device (I2CA)
        {
            Name (_HID, "AMDI0010")  // _HID: Hardware ID
            Name (_UID, Zero)  // _UID: Unique ID
            Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
            {
                IRQ (Edge, ActiveHigh, Exclusive, )
                    {10}
                Memory32Fixed (ReadWrite,
                    0xFEDC2000,         // Address Base
                    0x00001000,         // Address Length
                    )
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (TSOS, 0x70))
                {
                    If (LEqual (IC0E, One))
                    {
                        Return (0x0F)
                    }

                    Return (Zero)
                }
                Else
                {
                    Return (Zero)
                }
            }

            Method (RSET, 0, NotSerialized)
            {
                SRAD (0x05, 0xC8)
            }
        }

        Device (I2CB)
        {
            Name (_HID, "AMDI0010")  // _HID: Hardware ID
            Name (_UID, One)  // _UID: Unique ID
            Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
            {
                IRQ (Edge, ActiveHigh, Exclusive, )
                    {11}
                Memory32Fixed (ReadWrite,
                    0xFEDC3000,         // Address Base
                    0x00001000,         // Address Length
                    )
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (TSOS, 0x70))
                {
                    If (LEqual (IC1E, One))
                    {
                        Return (0x0F)
                    }

                    Return (Zero)
                }
                Else
                {
                    Return (Zero)
                }
            }

            Method (RSET, 0, NotSerialized)
            {
                SRAD (0x06, 0xC8)
            }
        }

        Device (I2CC)
        {
            Name (_HID, "AMDI0010")  // _HID: Hardware ID
            Name (_UID, 0x02)  // _UID: Unique ID
            Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
            {
                IRQ (Edge, ActiveHigh, Exclusive, )
                    {4}
                Memory32Fixed (ReadWrite,
                    0xFEDC4000,         // Address Base
                    0x00001000,         // Address Length
                    )
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (TSOS, 0x70))
                {
                    If (LEqual (IC2E, One))
                    {
                        Return (0x0F)
                    }

                    Return (Zero)
                }
                Else
                {
                    Return (Zero)
                }
            }

            Method (RSET, 0, NotSerialized)
            {
                SRAD (0x07, 0xC8)
            }
        }

        Device (I2CD)
        {
            Name (_HID, "AMDI0010")  // _HID: Hardware ID
            Name (_UID, 0x03)  // _UID: Unique ID
            Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
            {
                IRQ (Edge, ActiveHigh, Exclusive, )
                    {6}
                Memory32Fixed (ReadWrite,
                    0xFEDC5000,         // Address Base
                    0x00001000,         // Address Length
                    )
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (0x0F)
            }

            Method (RSET, 0, NotSerialized)
            {
                SRAD (0x08, 0xC8)
            }
        }

        Device (I2CE)
        {
            Name (_HID, "AMDI0010")  // _HID: Hardware ID
            Name (_UID, 0x04)  // _UID: Unique ID
            Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
            {
                IRQ (Edge, ActiveHigh, Exclusive, )
                    {14}
                Memory32Fixed (ReadWrite,
                    0xFEDC6000,         // Address Base
                    0x00001000,         // Address Length
                    )
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (TSOS, 0x70))
                {
                    If (LEqual (IC4E, One))
                    {
                        Return (0x0F)
                    }

                    Return (Zero)
                }
                Else
                {
                    Return (Zero)
                }
            }

            Method (RSET, 0, NotSerialized)
            {
                SRAD (0x09, 0xC8)
            }
        }

        Device (I2CF)
        {
            Name (_HID, "AMDI0010")  // _HID: Hardware ID
            Name (_UID, 0x05)  // _UID: Unique ID
            Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
            {
                IRQ (Edge, ActiveHigh, Exclusive, )
                    {15}
                Memory32Fixed (ReadWrite,
                    0xFEDCB000,         // Address Base
                    0x00001000,         // Address Length
                    )
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (TSOS, 0x70))
                {
                    If (LEqual (IC5E, One))
                    {
                        Return (0x0F)
                    }

                    Return (Zero)
                }
                Else
                {
                    Return (Zero)
                }
            }

            Method (RSET, 0, NotSerialized)
            {
                SRAD (0x0A, 0xC8)
            }
        }

        Method (EPIN, 0, NotSerialized)
        {
            Store (Zero, IPDE) /* \_SB_.IPDE */
            Store (Zero, IMPE) /* \_SB_.IMPE */
            Store (One, IM15) /* \_SB_.IM15 */
            Store (One, IM16) /* \_SB_.IM16 */
            Store (One, IM20) /* \_SB_.IM20 */
            Store (One, IM44) /* \_SB_.IM44 */
            Store (One, IM46) /* \_SB_.IM46 */
            Store (One, IM68) /* \_SB_.IM68 */
            Store (One, IM69) /* \_SB_.IM69 */
            Store (One, IM6A) /* \_SB_.IM6A */
            Store (One, IM6B) /* \_SB_.IM6B */
            Store (One, IM4A) /* \_SB_.IM4A */
            Store (One, IM58) /* \_SB_.IM58 */
            Store (One, IM4B) /* \_SB_.IM4B */
            Store (One, IM57) /* \_SB_.IM57 */
            Store (One, IM6D) /* \_SB_.IM6D */
            Store (One, IM1F) /* \_SB_.IM1F */
            SECR ()
        }

        Name (NCRS, ResourceTemplate ()
        {
            Interrupt (ResourceConsumer, Level, ActiveLow, Shared, ,, )
            {
                0x00000005,
            }
            Memory32Fixed (ReadWrite,
                0xFEDD5000,         // Address Base
                0x00001000,         // Address Length
                )
        })
        Name (DCRS, ResourceTemplate ()
        {
            Interrupt (ResourceConsumer, Level, ActiveLow, Shared, ,, )
            {
                0x00000005,
            }
            Memory32Fixed (ReadWrite,
                0xFEDD5000,         // Address Base
                0x00001000,         // Address Length
                )
            GpioInt (Edge, ActiveBoth, SharedAndWake, PullUp, 0x0BB8,
                "\\_SB.GPIO", 0x00, ResourceConsumer, ,
                )
                {   // Pin list
                    0x0044
                }
            GpioIo (Shared, PullUp, 0x0000, 0x0000, IoRestrictionNone,
                "\\_SB.GPIO", 0x00, ResourceConsumer, ,
                )
                {   // Pin list
                    0x0044
                }
        })
        Name (AHID, "AMDI0040")
        Name (ACID, "AMDI0040")
        Name (SHID, 0x400DD041)
        Name (SCID, "PCICC_080501")
        Device (EMM0)
        {
            Method (_HID, 0, Serialized)  // _HID: Hardware ID
            {
                If (EMMD)
                {
                    Return (SHID) /* \_SB_.SHID */
                }
                Else
                {
                    Return (AHID) /* \_SB_.AHID */
                }
            }

            Method (_CID, 0, Serialized)  // _CID: Compatible ID
            {
                If (EMMD)
                {
                    Return (SCID) /* \_SB_.SCID */
                }
                Else
                {
                    Return (ACID) /* \_SB_.ACID */
                }
            }

            Name (_UID, Zero)  // _UID: Unique ID
            Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
            {
                If (EMD3)
                {
                    Return (DCRS) /* \_SB_.DCRS */
                }
                Else
                {
                    Return (NCRS) /* \_SB_.NCRS */
                }
            }

            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (TSOS, 0x70))
                {
                    If (EMME)
                    {
                        Return (0x0F)
                    }

                    Return (Zero)
                }
                Else
                {
                    Return (Zero)
                }
            }

            Method (_INI, 0, NotSerialized)  // _INI: Initialize
            {
                If (EMME)
                {
                    EPIN ()
                }
            }

            Method (_S0W, 0, NotSerialized)  // _S0W: S0 Device Wake State
            {
                If (LAnd (EMD3, EMME))
                {
                    Return (0x04)
                }
                Else
                {
                    Return (Zero)
                }
            }

            Method (_PS0, 0, NotSerialized)  // _PS0: Power State 0
            {
                If (LAnd (EMD3, EMME))
                {
                    HSAD (0x1C, Zero)
                    RECR ()
                }
            }

            Method (_PS3, 0, NotSerialized)  // _PS3: Power State 3
            {
                If (LAnd (EMD3, EMME))
                {
                    HSAD (0x1C, 0x03)
                }
            }
        }
    }

    Scope (_SB.PCI0)
    {
        Device (UAR1)
        {
            Name (_HID, EisaId ("PNP0500") /* Standard PC COM Serial Port */)  // _HID: Hardware ID
            Name (_DDN, "COM1")  // _DDN: DOS Device Name
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (IER0)
                {
                    Return (0x0F)
                }

                Return (Zero)
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                Name (BUF0, ResourceTemplate ()
                {
                    IO (Decode16,
                        0x02E8,             // Range Minimum
                        0x02E8,             // Range Maximum
                        0x01,               // Alignment
                        0x08,               // Length
                        _Y18)
                    IRQNoFlags (_Y19)
                        {3}
                })
                CreateByteField (BUF0, \_SB.PCI0.UAR1._CRS._Y18._MIN, IOLO)  // _MIN: Minimum Base Address
                CreateByteField (BUF0, 0x03, IOHI)
                CreateByteField (BUF0, \_SB.PCI0.UAR1._CRS._Y18._MAX, IORL)  // _MAX: Maximum Base Address
                CreateByteField (BUF0, 0x05, IORH)
                CreateWordField (BUF0, \_SB.PCI0.UAR1._CRS._Y19._INT, IRQL)  // _INT: Interrupts
                FRUI (WUR0, IRQL)
                Return (BUF0) /* \_SB_.PCI0.UAR1._CRS.BUF0 */
            }
        }

        Device (UAR2)
        {
            Name (_HID, EisaId ("PNP0500") /* Standard PC COM Serial Port */)  // _HID: Hardware ID
            Name (_DDN, "COM2")  // _DDN: DOS Device Name
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (IER1)
                {
                    Return (0x0F)
                }

                Return (Zero)
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                Name (BUF0, ResourceTemplate ()
                {
                    IO (Decode16,
                        0x02F8,             // Range Minimum
                        0x02F8,             // Range Maximum
                        0x01,               // Alignment
                        0x08,               // Length
                        _Y1A)
                    IRQNoFlags (_Y1B)
                        {4}
                })
                CreateByteField (BUF0, \_SB.PCI0.UAR2._CRS._Y1A._MIN, IOLO)  // _MIN: Minimum Base Address
                CreateByteField (BUF0, 0x03, IOHI)
                CreateByteField (BUF0, \_SB.PCI0.UAR2._CRS._Y1A._MAX, IORL)  // _MAX: Maximum Base Address
                CreateByteField (BUF0, 0x05, IORH)
                CreateWordField (BUF0, \_SB.PCI0.UAR2._CRS._Y1B._INT, IRQL)  // _INT: Interrupts
                FRUI (WUR1, IRQL)
                Return (BUF0) /* \_SB_.PCI0.UAR2._CRS.BUF0 */
            }
        }

        Device (UAR3)
        {
            Name (_HID, EisaId ("PNP0500") /* Standard PC COM Serial Port */)  // _HID: Hardware ID
            Name (_DDN, "COM3")  // _DDN: DOS Device Name
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (IER2)
                {
                    Return (0x0F)
                }

                Return (Zero)
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                Name (BUF0, ResourceTemplate ()
                {
                    IO (Decode16,
                        0x03E8,             // Range Minimum
                        0x03E8,             // Range Maximum
                        0x01,               // Alignment
                        0x08,               // Length
                        _Y1C)
                    IRQNoFlags (_Y1D)
                        {3}
                })
                CreateByteField (BUF0, \_SB.PCI0.UAR3._CRS._Y1C._MIN, IOLO)  // _MIN: Minimum Base Address
                CreateByteField (BUF0, 0x03, IOHI)
                CreateByteField (BUF0, \_SB.PCI0.UAR3._CRS._Y1C._MAX, IORL)  // _MAX: Maximum Base Address
                CreateByteField (BUF0, 0x05, IORH)
                CreateWordField (BUF0, \_SB.PCI0.UAR3._CRS._Y1D._INT, IRQL)  // _INT: Interrupts
                FRUI (WUR2, IRQL)
                Return (BUF0) /* \_SB_.PCI0.UAR3._CRS.BUF0 */
            }
        }

        Device (UAR4)
        {
            Name (_HID, EisaId ("PNP0500") /* Standard PC COM Serial Port */)  // _HID: Hardware ID
            Name (_DDN, "COM4")  // _DDN: DOS Device Name
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (IER3)
                {
                    Return (0x0F)
                }

                Return (Zero)
            }

            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                Name (BUF0, ResourceTemplate ()
                {
                    IO (Decode16,
                        0x03F8,             // Range Minimum
                        0x03F8,             // Range Maximum
                        0x01,               // Alignment
                        0x08,               // Length
                        _Y1E)
                    IRQNoFlags (_Y1F)
                        {4}
                })
                CreateByteField (BUF0, \_SB.PCI0.UAR4._CRS._Y1E._MIN, IOLO)  // _MIN: Minimum Base Address
                CreateByteField (BUF0, 0x03, IOHI)
                CreateByteField (BUF0, \_SB.PCI0.UAR4._CRS._Y1E._MAX, IORL)  // _MAX: Maximum Base Address
                CreateByteField (BUF0, 0x05, IORH)
                CreateWordField (BUF0, \_SB.PCI0.UAR4._CRS._Y1F._INT, IRQL)  // _INT: Interrupts
                FRUI (WUR3, IRQL)
                Return (BUF0) /* \_SB_.PCI0.UAR4._CRS.BUF0 */
            }
        }
    }

    Scope (_SB.PCI0.GP18.SATA)
    {
        Name (_PR0, Package (0x01)  // _PR0: Power Resources for D0
        {
            P0S0
        })
        Name (_PR3, Package (0x01)  // _PR3: Power Resources for D3hot
        {
            P3S0
        })
        Method (_S0W, 0, NotSerialized)  // _S0W: S0 Device Wake State
        {
            If (LEqual (ST_D, One))
            {
                Return (0x04)
            }
            Else
            {
                Return (Zero)
            }
        }

        Method (_PS0, 0, NotSerialized)  // _PS0: Power State 0
        {
            If (LEqual (ST_D, One))
            {
                Store (0xD6, SMIW) /* \_SB_.SMIW */
            }
        }

        Method (_PS3, 0, NotSerialized)  // _PS3: Power State 3
        {
            If (LEqual (ST_D, One))
            {
                Store (0xD5, SMIW) /* \_SB_.SMIW */
            }
        }
    }

    Scope (_SB.PCI0.GP18.SAT1)
    {
        Name (_PR0, Package (0x01)  // _PR0: Power Resources for D0
        {
            P0S1
        })
        Name (_PR3, Package (0x01)  // _PR3: Power Resources for D3hot
        {
            P3S1
        })
        Method (_S0W, 0, NotSerialized)  // _S0W: S0 Device Wake State
        {
            If (LEqual (ST_D, One))
            {
                Return (0x04)
            }
            Else
            {
                Return (Zero)
            }
        }

        Method (_PS0, 0, NotSerialized)  // _PS0: Power State 0
        {
            If (LEqual (ST_D, One))
            {
                Store (0x01D6, SMIW) /* \_SB_.SMIW */
            }
        }

        Method (_PS3, 0, NotSerialized)  // _PS3: Power State 3
        {
            If (LEqual (ST_D, One))
            {
                Store (0x01D5, SMIW) /* \_SB_.SMIW */
            }
        }
    }

    Device (HPET)
    {
        Name (_HID, EisaId ("PNP0103") /* HPET System Timer */)  // _HID: Hardware ID
        Method (_STA, 0, NotSerialized)  // _STA: Status
        {
            If (LEqual (HPEN, One))
            {
                If (LGreaterEqual (OSVR, 0x0C))
                {
                    Return (0x0F)
                }

                Store (Zero, HPEN) /* \HPEN */
                Return (One)
            }

            Return (One)
        }

        Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
        {
            Name (BUF0, ResourceTemplate ()
            {
                IRQNoFlags ()
                    {0}
                IRQNoFlags ()
                    {8}
                Memory32Fixed (ReadOnly,
                    0xFED00000,         // Address Base
                    0x00000400,         // Address Length
                    )
            })
            Return (BUF0) /* \HPET._CRS.BUF0 */
        }
    }

    Device (_SB.TPM)
    {
        Name (TMRQ, 0xFFFFFFFF)
        Name (TLVL, 0xFFFFFFFF)
        Method (_HID, 0, NotSerialized)  // _HID: Hardware ID
        {
            If (TCMF)
            {
                Return (0x01013469)
            }
            ElseIf (LEqual (TTDP, Zero))
            {
                Return (0x310CD041)
            }
            Else
            {
                Return ("MSFT0101")
            }
        }

        OperationRegion (TMMB, SystemMemory, 0xFED40000, 0x5000)
        Field (TMMB, ByteAcc, Lock, Preserve)
        {
            ACC0,   8, 
            Offset (0x08), 
            INTE,   32, 
            INTV,   8, 
            Offset (0x10), 
            INTS,   32, 
            INTF,   32, 
            TSTS,   32, 
            Offset (0x24), 
            FIFO,   32, 
            Offset (0x30), 
            IDTF,   32, 
            Offset (0x4C), 
            SCMD,   32
        }

        Method (_STR, 0, NotSerialized)  // _STR: Description String
        {
            If (LEqual (TTDP, Zero))
            {
                Return (Unicode ("TPM 1.2 Device"))
            }
            Else
            {
                Return (Unicode ("TPM 2.0 Device"))
            }
        }

        Name (_UID, One)  // _UID: Unique ID
        Name (CRST, ResourceTemplate ()
        {
            Memory32Fixed (ReadOnly,
                0x00000000,         // Address Base
                0x00001000,         // Address Length
                _Y20)
            Memory32Fixed (ReadOnly,
                0xFED70000,         // Address Base
                0x00001000,         // Address Length
                _Y21)
        })
        Name (CRSD, ResourceTemplate ()
        {
            Memory32Fixed (ReadWrite,
                0xFED40000,         // Address Base
                0x00005000,         // Address Length
                _Y22)
        })
        Name (CRID, ResourceTemplate ()
        {
            Memory32Fixed (ReadWrite,
                0xFED40000,         // Address Base
                0x00005000,         // Address Length
                _Y23)
        })
        Name (CREI, ResourceTemplate ()
        {
            Memory32Fixed (ReadWrite,
                0xFED40000,         // Address Base
                0x00005000,         // Address Length
                )
            GpioInt (Level, ActiveLow, ExclusiveAndWake, PullNone, 0x0000,
                "\\_SB.GPIO", 0x00, ResourceConsumer, _Y24,
                )
                {   // Pin list
                    0x0000
                }
        })
        Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
        {
            If (LEqual (AMDT, One))
            {
                CreateDWordField (CRST, \_SB.TPM._Y20._BAS, MTFB)  // _BAS: Base Address
                CreateDWordField (CRST, \_SB.TPM._Y20._LEN, LTFB)  // _LEN: Length
                Store (TPMB, MTFB) /* \_SB_.TPM_._CRS.MTFB */
                Store (TPBS, LTFB) /* \_SB_.TPM_._CRS.LTFB */
                CreateDWordField (CRST, \_SB.TPM._Y21._BAS, MTFC)  // _BAS: Base Address
                CreateDWordField (CRST, \_SB.TPM._Y21._LEN, LTFC)  // _LEN: Length
                Store (TPMC, MTFC) /* \_SB_.TPM_._CRS.MTFC */
                Store (TPCS, LTFC) /* \_SB_.TPM_._CRS.LTFC */
                Return (CRST) /* \_SB_.TPM_.CRST */
            }
            Else
            {
                If (LEqual (DTPT, One))
                {
                    CreateDWordField (CRSD, \_SB.TPM._Y22._BAS, MTFE)  // _BAS: Base Address
                    CreateDWordField (CRSD, \_SB.TPM._Y22._LEN, LTFE)  // _LEN: Length
                    Store (0xFED40000, MTFE) /* \_SB_.TPM_._CRS.MTFE */
                    Store (0x5000, LTFE) /* \_SB_.TPM_._CRS.LTFE */
                    Return (CRSD) /* \_SB_.TPM_.CRSD */
                }
                ElseIf (LEqual (TTPF, One))
                {
                    If (LAnd (LEqual (TMRQ, Zero), LNotEqual (TMRQ, 0xFFFFFFFF)))
                    {
                        CreateDWordField (CRID, \_SB.TPM._Y23._BAS, MTFD)  // _BAS: Base Address
                        CreateDWordField (CRID, \_SB.TPM._Y23._LEN, LTFD)  // _LEN: Length
                        Store (0xFED40000, MTFD) /* \_SB_.TPM_._CRS.MTFD */
                        Store (0x5000, LTFD) /* \_SB_.TPM_._CRS.LTFD */
                        Return (CRID) /* \_SB_.TPM_.CRID */
                    }
                    Else
                    {
                        CreateWordField (CREI, 0x23, LIRQ)
                        CreateBitField (CREI, \_SB.TPM._Y24._POL, LLVL)  // _POL: Polarity
                        Store (TMRQ, LIRQ) /* \_SB_.TPM_._CRS.LIRQ */
                        Store (TLVL, LLVL) /* \_SB_.TPM_._CRS.LLVL */
                        Return (CREI) /* \_SB_.TPM_.CREI */
                    }
                }
                ElseIf (LEqual (TTPF, Zero))
                {
                    CreateDWordField (CRST, \_SB.TPM._Y21._BAS, MTFF)  // _BAS: Base Address
                    Store (FTPM, MTFF) /* \_SB_.TPM_._CRS.MTFF */
                    Return (CRST) /* \_SB_.TPM_.CRST */
                }

                Store (Zero, MTFE) /* \_SB_.TPM_._CRS.MTFE */
                Store (Zero, LTFE) /* \_SB_.TPM_._CRS.LTFE */
                Return (CRID) /* \_SB_.TPM_.CRID */
            }

            Return (CRID) /* \_SB_.TPM_.CRID */
        }

        Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
        {
            If (LAnd (LNotEqual (TMRQ, Zero), LNotEqual (TMRQ, 0xFFFFFFFF)))
            {
                CreateWordField (Arg0, 0x23, IRQ0)
                CreateWordField (CREI, 0x23, LIRQ)
                Store (IRQ0, LIRQ) /* \_SB_.TPM_._SRS.LIRQ */
                Store (IRQ0, TMRQ) /* \_SB_.TPM_.TMRQ */
                CreateBitField (Arg0, 0x98, ITRG)
                CreateBitField (CREI, \_SB.TPM._Y24._MOD, LTRG)  // _MOD: Mode
                Store (ITRG, LTRG) /* \_SB_.TPM_._SRS.LTRG */
                CreateBitField (Arg0, 0x99, ILVL)
                CreateBitField (CREI, \_SB.TPM._Y24._POL, LLVL)  // _POL: Polarity
                Store (ILVL, LLVL) /* \_SB_.TPM_._SRS.LLVL */
                If (LOr (LEqual (And (IDTF, 0x0F), Zero), LEqual (And (IDTF, 0x0F
                    ), 0x0F)))
                {
                    If (LLess (IRQ0, 0x10))
                    {
                        Store (And (IRQ0, 0x0F), INTV) /* \_SB_.TPM_.INTV */
                    }

                    If (LEqual (ITRG, One))
                    {
                        Or (INTE, 0x10, INTE) /* \_SB_.TPM_.INTE */
                    }
                    Else
                    {
                        And (INTE, 0xFFFFFFEF, INTE) /* \_SB_.TPM_.INTE */
                    }

                    If (LEqual (ILVL, Zero))
                    {
                        Or (INTE, 0x08, INTE) /* \_SB_.TPM_.INTE */
                    }
                    Else
                    {
                        And (INTE, 0xFFFFFFF7, INTE) /* \_SB_.TPM_.INTE */
                    }
                }
            }
        }

        OperationRegion (CRBD, SystemMemory, TPMM, 0x48)
        Field (CRBD, AnyAcc, NoLock, Preserve)
        {
            Offset (0x04), 
            HERR,   32, 
            Offset (0x40), 
            HCMD,   32, 
            HSTS,   32
        }

        Method (_STA, 0, NotSerialized)  // _STA: Status
        {
            If (LEqual (TTDP, Zero))
            {
                If (TPMF)
                {
                    Return (0x0F)
                }

                Return (Zero)
            }
            ElseIf (LEqual (TTDP, One))
            {
                If (TPMF)
                {
                    Return (0x0F)
                }

                Return (Zero)
            }

            Return (Zero)
        }

        Method (STRT, 3, Serialized)
        {
            OperationRegion (TPMR, SystemMemory, FTPM, 0x1000)
            Field (TPMR, AnyAcc, NoLock, Preserve)
            {
                Offset (0x04), 
                FERR,   32, 
                Offset (0x0C), 
                BEGN,   32
            }

            Name (TIMR, Zero)
            If (LNotEqual (ToInteger (Arg0), Zero)){}
            Switch (ToInteger (Arg1))
            {
                Case (Zero)
                {
                    Return (Buffer (One)
                    {
                         0x03                                             // .
                    })
                }
                Case (One)
                {
                    Store (Zero, TIMR) /* \_SB_.TPM_.STRT.TIMR */
                    If (LEqual (AMDT, One))
                    {
                        While (LAnd (LEqual (BEGN, One), LLess (TIMR, 0x0200)))
                        {
                            If (LEqual (BEGN, One))
                            {
                                Sleep (One)
                                Increment (TIMR)
                            }
                        }
                    }
                    ElseIf (LEqual (Or (And (HSTS, 0x02), And (HSTS, One)
                        ), 0x03))
                    {
                        Store (One, HCMD) /* \_SB_.TPM_.HCMD */
                    }
                    Else
                    {
                        Store (One, FERR) /* \_SB_.TPM_.STRT.FERR */
                        Store (Zero, BEGN) /* \_SB_.TPM_.STRT.BEGN */
                    }

                    Return (Zero)
                }

            }

            Return (One)
        }

        Method (CRYF, 3, Serialized)
        {
            If (LNotEqual (ToInteger (Arg0), One)){}
            Switch (ToInteger (Arg1))
            {
                Case (Zero)
                {
                    Return (Buffer (One)
                    {
                         0x03                                             // .
                    })
                }
                Case (One)
                {
                    Name (TPMV, Package (0x02)
                    {
                        One, 
                        Package (0x02)
                        {
                            One, 
                            0x20
                        }
                    })
                    If (LEqual (_STA (), Zero))
                    {
                        Return (Package (0x01)
                        {
                            Zero
                        })
                    }

                    Return (TPMV) /* \_SB_.TPM_.CRYF.TPMV */
                }

            }

            Return (Buffer (One)
            {
                 0x00                                             // .
            })
        }
    }

    Scope (_SB.TPM)
    {
        OperationRegion (TSMI, SystemIO, SMIA, 0x02)
        Field (TSMI, WordAcc, NoLock, Preserve)
        {
            SMI,    16
        }

        OperationRegion (ATNV, SystemMemory, PPIM, PPIL)
        Field (ATNV, AnyAcc, NoLock, Preserve)
        {
            RQST,   32, 
            RCNT,   32, 
            ERRO,   32, 
            FLAG,   32, 
            MISC,   32, 
            OPTN,   32, 
            SRSP,   32
        }

        Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
        {
            If (LEqual (Arg0, ToUUID ("3dddfaa6-361b-4eb4-a424-8d10089d1653") /* Physical Presence Interface */))
            {
                Switch (ToInteger (Arg2))
                {
                    Case (Zero)
                    {
                        Return (Buffer (0x02)
                        {
                             0xFF, 0x01                                       // ..
                        })
                    }
                    Case (One)
                    {
                        If (LEqual (PPIV, Zero))
                        {
                            Return ("1.2")
                        }
                        Else
                        {
                            Return ("1.3")
                        }
                    }
                    Case (0x02)
                    {
                        Store (DerefOf (Index (Arg3, Zero)), RQST) /* \_SB_.TPM_.RQST */
                        Store (Zero, SRSP) /* \_SB_.TPM_.SRSP */
                        Store (0x02, FLAG) /* \_SB_.TPM_.FLAG */
                        Store (OFST, TMF1) /* \TMF1 */
                        Store (Zero, SRSP) /* \_SB_.TPM_.SRSP */
                        Store (TMF1, SMI) /* \_SB_.TPM_.SMI_ */
                        Return (SRSP) /* \_SB_.TPM_.SRSP */
                    }
                    Case (0x03)
                    {
                        Name (PPI1, Package (0x02)
                        {
                            Zero, 
                            Zero
                        })
                        Store (RQST, Index (PPI1, One))
                        Return (PPI1) /* \_SB_.TPM_._DSM.PPI1 */
                    }
                    Case (0x04)
                    {
                        Return (TRST) /* \TRST */
                    }
                    Case (0x05)
                    {
                        Name (PPI2, Package (0x03)
                        {
                            Zero, 
                            Zero, 
                            Zero
                        })
                        Store (Zero, SRSP) /* \_SB_.TPM_.SRSP */
                        Store (0x05, FLAG) /* \_SB_.TPM_.FLAG */
                        Store (OFST, SMI) /* \_SB_.TPM_.SMI_ */
                        Store (RCNT, Index (PPI2, One))
                        Store (ERRO, Index (PPI2, 0x02))
                        Return (PPI2) /* \_SB_.TPM_._DSM.PPI2 */
                    }
                    Case (0x06)
                    {
                        Return (0x03)
                    }
                    Case (0x07)
                    {
                        Store (DerefOf (Index (Arg3, Zero)), RQST) /* \_SB_.TPM_.RQST */
                        Store (0x07, FLAG) /* \_SB_.TPM_.FLAG */
                        Store (Zero, OPTN) /* \_SB_.TPM_.OPTN */
                        If (LEqual (RQST, 0x17))
                        {
                            ToInteger (DerefOf (Index (Arg3, One)), OPTN) /* \_SB_.TPM_.OPTN */
                        }

                        Store (OFST, TMF1) /* \TMF1 */
                        Store (Zero, SRSP) /* \_SB_.TPM_.SRSP */
                        Store (TMF1, SMI) /* \_SB_.TPM_.SMI_ */
                        Return (SRSP) /* \_SB_.TPM_.SRSP */
                    }
                    Case (0x08)
                    {
                        Store (DerefOf (Index (Arg3, Zero)), RQST) /* \_SB_.TPM_.RQST */
                        Store (0x08, FLAG) /* \_SB_.TPM_.FLAG */
                        Store (OFST, TMF1) /* \TMF1 */
                        Store (Zero, SRSP) /* \_SB_.TPM_.SRSP */
                        Store (TMF1, SMI) /* \_SB_.TPM_.SMI_ */
                        Return (SRSP) /* \_SB_.TPM_.SRSP */
                    }
                    Default
                    {
                    }

                }
            }
            ElseIf (LEqual (Arg0, ToUUID ("376054ed-cc13-4675-901c-4756d7f2d45d")))
            {
                Switch (ToInteger (Arg2))
                {
                    Case (Zero)
                    {
                        Return (Buffer (One)
                        {
                             0x03                                             // .
                        })
                    }
                    Case (One)
                    {
                        Store (DerefOf (Index (Arg3, Zero)), RQST) /* \_SB_.TPM_.RQST */
                        Store (0x09, FLAG) /* \_SB_.TPM_.FLAG */
                        Store (OFST, TMF1) /* \TMF1 */
                        Store (Zero, SRSP) /* \_SB_.TPM_.SRSP */
                        Store (TMF1, SMI) /* \_SB_.TPM_.SMI_ */
                        Return (SRSP) /* \_SB_.TPM_.SRSP */
                    }
                    Default
                    {
                    }

                }
            }

            If (LEqual (Arg0, ToUUID ("cf8e16a5-c1e8-4e25-b712-4f54a96702c8")))
            {
                Return (CRYF (Arg1, Arg2, Arg3))
            }

            If (LEqual (Arg0, ToUUID ("6bbf6cab-5463-4714-b7cd-f0203c0368d4")))
            {
                Return (STRT (Arg1, Arg2, Arg3))
            }

            Return (Buffer (One)
            {
                 0x00                                             // .
            })
        }

        Method (TPTS, 1, Serialized)
        {
            Switch (ToInteger (Arg0))
            {
                Case (0x04)
                {
                    Store (Zero, RQST) /* \_SB_.TPM_.RQST */
                    Store (0x09, FLAG) /* \_SB_.TPM_.FLAG */
                    Store (Zero, SRSP) /* \_SB_.TPM_.SRSP */
                    Store (OFST, SMI) /* \_SB_.TPM_.SMI_ */
                }
                Case (0x05)
                {
                    Store (Zero, RQST) /* \_SB_.TPM_.RQST */
                    Store (0x09, FLAG) /* \_SB_.TPM_.FLAG */
                    Store (Zero, SRSP) /* \_SB_.TPM_.SRSP */
                    Store (OFST, SMI) /* \_SB_.TPM_.SMI_ */
                }

            }

            Sleep (0x012C)
        }
    }

    Scope (_SB)
    {
        OperationRegion (RAMW, SystemMemory, 0xAC572000, 0x0100)
        Field (RAMW, AnyAcc, NoLock, Preserve)
        {
            AMLS,   32, 
            ARS1,   32, 
            ARS2,   32, 
            ARS3,   32, 
            APKG,   32, 
            AMDL,   32, 
            ACPB,   32, 
            ACAB,   32, 
            AECB,   32, 
            ALBB,   32, 
            APSB,   32, 
            ASRB,   32, 
            ASIB,   32, 
            AUSB,   32, 
            AVGB,   32, 
            AWRB,   32, 
            ARSB,   32, 
            ADPB,   32
        }
    }

    Scope (_SB)
    {
        OperationRegion (APKO, SystemMemory, APKG, 0x04)
        Field (APKO, AnyAcc, NoLock, Preserve)
        {
            APKS,   32
        }
    }

    Scope (_SB)
    {
        OperationRegion (AMDO, SystemMemory, AMDL, 0x04)
        Field (AMDO, AnyAcc, NoLock, Preserve)
        {
            AMDS,   32
        }
    }

    Scope (_SB)
    {
        OperationRegion (ACPO, SystemMemory, ACPB, 0x0E)
        Field (ACPO, AnyAcc, NoLock, Preserve)
        {
            ACPS,   32, 
            SMIF,   8, 
            ALPR,   32, 
            TMPB,   8, 
            INSK,   8, 
            WLDP,   8, 
            BTDP,   8, 
            APSC,   8
        }
    }

    Scope (_SB)
    {
        OperationRegion (ACAO, SystemMemory, ACAB, 0x10)
        Field (ACAO, AnyAcc, NoLock, Preserve)
        {
            ACAS,   32, 
            CAL1,   32, 
            CA10,   32, 
            CA11,   32
        }
    }

    Scope (_SB)
    {
        OperationRegion (AECO, SystemMemory, AECB, 0x14)
        Field (AECO, AnyAcc, NoLock, Preserve)
        {
            AECS,   32, 
            ACPF,   8, 
            DCPF,   8, 
            DCP2,   8, 
            TPSV,   8, 
            TCRT,   8, 
            PPSV,   8, 
            DSYN,   8, 
            WOLO,   8, 
            EGBF,   32, 
            KBLC,   8, 
            KBLV,   8, 
            ALAE,   8, 
            KFSK,   8
        }
    }

    Scope (_SB)
    {
        OperationRegion (ALBO, SystemMemory, ALBB, 0x04)
        Field (ALBO, AnyAcc, NoLock, Preserve)
        {
            ALBS,   32
        }
    }

    Scope (_SB)
    {
        OperationRegion (APSO, SystemMemory, APSB, 0x04)
        Field (APSO, AnyAcc, NoLock, Preserve)
        {
            APSS,   32
        }
    }

    Scope (_SB)
    {
        OperationRegion (ASRO, SystemMemory, ASRB, 0x04)
        Field (ASRO, AnyAcc, NoLock, Preserve)
        {
            ASRS,   32
        }
    }

    Scope (_SB)
    {
        OperationRegion (ASIO, SystemMemory, ASIB, 0x09)
        Field (ASIO, AnyAcc, NoLock, Preserve)
        {
            ASIS,   32, 
            TPIF,   8, 
            TPDI,   8, 
            TPLP,   8, 
            TPLI,   8, 
            TPHI,   8
        }
    }

    Scope (_SB)
    {
        OperationRegion (AUSO, SystemMemory, AUSB, 0x06)
        Field (AUSO, AnyAcc, NoLock, Preserve)
        {
            AUBS,   32, 
            VBOF,   16
        }
    }

    Scope (_SB)
    {
        OperationRegion (AVGO, SystemMemory, AVGB, 0x12)
        Field (AVGO, AnyAcc, NoLock, Preserve)
        {
            AVGS,   32, 
            VGAF,   16, 
            LCDV,   32, 
            LCDR,   8, 
            LCDS,   32, 
            ACTD,   8, 
            AVLD,   8, 
            SETD,   8
        }
    }

    Scope (_SB)
    {
        OperationRegion (AURO, SystemMemory, AWRB, 0x1D)
        Field (AURO, AnyAcc, NoLock, Preserve)
        {
            STXE,   8, 
            ST10,   8, 
            ST11,   8, 
            ST12,   8, 
            ST13,   8, 
            ST14,   8, 
            ST15,   8, 
            ST16,   8, 
            ST17,   8, 
            ST18,   8, 
            ST19,   8, 
            SD11,   8, 
            SD12,   8, 
            SD13,   8, 
            SD14,   8, 
            SD15,   8, 
            SD16,   8, 
            SD21,   8, 
            SD22,   8, 
            SD23,   8, 
            SD24,   8, 
            SD25,   8, 
            SD26,   8, 
            SD31,   8, 
            SD32,   8, 
            SD33,   8, 
            SD34,   8, 
            SD35,   8, 
            SD36,   8
        }
    }

    Scope (_SB)
    {
        OperationRegion (ARSO, SystemMemory, ARSB, 0x05)
        Field (ARSO, AnyAcc, NoLock, Preserve)
        {
            ARSS,   32, 
            ARSE,   8
        }
    }

    Scope (_SB)
    {
        OperationRegion (ADPO, SystemMemory, ADPB, 0x05)
        Field (ADPO, AnyAcc, NoLock, Preserve)
        {
            ADPS,   32, 
            CPFG,   8
        }
    }

    Scope (\)
    {
        Method (DIAG, 1, NotSerialized)
        {
        }

        OperationRegion (GPSC, SystemIO, 0xB2, 0x02)
        Field (GPSC, ByteAcc, NoLock, Preserve)
        {
            SMCM,   8, 
            SMST,   8
        }

        Method (ISMI, 1, Serialized)
        {
            Store (Arg0, SMCM) /* \SMCM */
        }

        Method (ASMI, 1, Serialized)
        {
            Store (Arg0, \_SB.ALPR)
            Store (0xA3, SMCM) /* \SMCM */
            Return (\_SB.ALPR)
        }

        Name (OSFG, Zero)
        Name (OS9X, One)
        Name (OS98, 0x02)
        Name (OSME, 0x04)
        Name (OS2K, 0x08)
        Name (OSXP, 0x10)
        Name (OSEG, 0x20)
        Name (OSVT, 0x40)
        Name (OSW7, 0x80)
        Name (OSW8, 0x0100)
        Name (OS13, 0x0110)
        Name (OS14, 0x0120)
        Name (OS15, 0x0130)
        Name (OS16, 0x0140)
        Name (OS17, 0x0150)
        Method (MSOS, 0, NotSerialized)
        {
            Return (0x0110)
        }

        Method (ADVG, 0, NotSerialized)
        {
            Return (0x03)
        }

        Method (GCDM, 0, NotSerialized)
        {
            Return (One)
        }

        Method (SWHG, 1, Serialized)
        {
            Return (Zero)
        }

        Method (NATK, 0, NotSerialized)
        {
            Return (One)
        }
    }

    Scope (_SB)
    {
        Device (SLPB)
        {
            Name (_HID, EisaId ("PNP0C0E") /* Sleep Button Device */)  // _HID: Hardware ID
            Name (_STA, 0x0B)  // _STA: Status
        }

        OperationRegion (AGPO, SystemMemory, 0xFEC30000, 0x1000)
        Field (AGPO, AnyAcc, NoLock, Preserve)
        {
            Offset (0x08), 
            GPOL,   8
        }

        Method (GGOV, 2, Serialized)
        {
            If (Arg0)
            {
                Return (ShiftRight (And (GPOL, ShiftLeft (One, Arg1)), Arg1))
            }
            Else
            {
                Return (M009 (Arg1))
            }
        }

        Method (SGOV, 3, Serialized)
        {
            If (Arg0)
            {
                Or (And (GPOL, Not (ShiftLeft (One, Arg1))), ShiftLeft (Arg2, 
                    Arg1), Local0)
                Store (Local0, GPOL) /* \_SB_.GPOL */
            }
            Else
            {
                M010 (Arg1, Arg2)
            }
        }

        Method (GPC0, 2, Serialized)
        {
            If (Arg0)
            {
                Return (ShiftRight (And (GPOL, ShiftLeft (One, Arg1)), Arg1))
            }
            Else
            {
                Return (M900 (Arg1))
            }
        }
    }

    Scope (_SB)
    {
        Name (ATKP, Zero)
        Name (AITM, Zero)
        Name (PLMD, Zero)
        Name (MUTX, One)
        Name (LEDS, Zero)
        Name (FNIV, Zero)
        Device (ATKD)
        {
            Name (_HID, "PNP0C14" /* Windows Management Instrumentation Device */)  // _HID: Hardware ID
            Name (_UID, "ATK")  // _UID: Unique ID
            Name (ATKQ, Package (0x10)
            {
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF
            })
            Name (AQHI, Zero)
            Name (AQTI, 0x0F)
            Name (AQNO, Zero)
            Method (IANQ, 1, Serialized)
            {
                If (LGreaterEqual (AQNO, 0x10))
                {
                    Store (0x64, Local0)
                    While (LAnd (Local0, LGreaterEqual (AQNO, 0x10)))
                    {
                        Decrement (Local0)
                        Sleep (0x0A)
                    }

                    If (LAnd (LNot (Local0), LGreaterEqual (AQNO, 0x10)))
                    {
                        Return (Zero)
                    }
                }

                Increment (AQTI)
                And (AQTI, 0x0F, AQTI) /* \_SB_.ATKD.AQTI */
                Store (Arg0, Index (ATKQ, AQTI))
                Increment (AQNO)
                Return (One)
            }

            Method (GANQ, 0, Serialized)
            {
                If (AQNO)
                {
                    Decrement (AQNO)
                    Store (DerefOf (Index (ATKQ, AQHI)), Local0)
                    Increment (AQHI)
                    And (AQHI, 0x0F, AQHI) /* \_SB_.ATKD.AQHI */
                    Return (Local0)
                }

                Return (Ones)
            }

            Name (_WDG, Buffer (0x3C)
            {
                /* 0000 */  0xD0, 0x5E, 0x84, 0x97, 0x6D, 0x4E, 0xDE, 0x11,  // .^..mN..
                /* 0008 */  0x8A, 0x39, 0x08, 0x00, 0x20, 0x0C, 0x9A, 0x66,  // .9.. ..f
                /* 0010 */  0x4E, 0x42, 0x01, 0x02, 0x35, 0xBB, 0x3C, 0x0B,  // NB..5.<.
                /* 0018 */  0xC2, 0xE3, 0xED, 0x45, 0x91, 0xC2, 0x4C, 0x5A,  // ...E..LZ
                /* 0020 */  0x6D, 0x19, 0x5D, 0x1C, 0xFF, 0x00, 0x01, 0x08,  // m.].....
                /* 0028 */  0x21, 0x12, 0x90, 0x05, 0x66, 0xD5, 0xD1, 0x11,  // !...f...
                /* 0030 */  0xB2, 0xF0, 0x00, 0xA0, 0xC9, 0x06, 0x29, 0x10,  // ......).
                /* 0038 */  0x4D, 0x4F, 0x01, 0x00                           // MO..
            })
            Scope (\_SB.ATKD)
            {
                Method (WLED, 1, NotSerialized)
                {
                    SGOV (One, One, Arg0)
                    XOr (Arg0, One, Arg0)
                    If (LEqual (LAnd (WAPF, 0x05), Zero)){}
                    Return (One)
                }

                Method (BLED, 1, NotSerialized)
                {
                    SGOV (One, One, Arg0)
                    Return (One)
                }

                Name (WAPF, Zero)
                Method (CWAP, 1, NotSerialized)
                {
                    Or (Arg0, WAPF, WAPF) /* \_SB_.ATKD.WAPF */
                    Return (One)
                }
            }

            Scope (\_SB.ATKD)
            {
            }

            Scope (\_SB.ATKD)
            {
                Method (OFBD, 1, NotSerialized)
                {
                    Name (FBDT, Package (0x53)
                    {
                        0x88, 
                        0x89, 
                        0x8A, 
                        0x8B, 
                        0x8C, 
                        0x8D, 
                        0x8E, 
                        0x8F, 
                        0x70, 
                        0x71, 
                        0x72, 
                        0x73, 
                        0x74, 
                        0x75, 
                        0x76, 
                        0x77, 
                        0x78, 
                        0x79, 
                        0x7A, 
                        0x7B, 
                        0x7C, 
                        0x7D, 
                        0x7E, 
                        0x7F, 
                        0x60, 
                        0x61, 
                        0x62, 
                        0x63, 
                        0x64, 
                        0x65, 
                        0x66, 
                        0x67, 
                        0x91, 
                        0x92, 
                        0x93, 
                        0x96, 
                        0xE0, 
                        0xE1, 
                        0xE2, 
                        0xE3, 
                        0xE4, 
                        0xE5, 
                        0xE6, 
                        0xE7, 
                        0xE8, 
                        0xE9, 
                        0xEA, 
                        0xEB, 
                        0xEC, 
                        0xED, 
                        0xEE, 
                        0xEF, 
                        0xD0, 
                        0xD1, 
                        0xD2, 
                        0xD3, 
                        0xD4, 
                        0xD5, 
                        0xD6, 
                        0xD7, 
                        0xD8, 
                        0xD9, 
                        0xDA, 
                        0xDB, 
                        0xDC, 
                        0xDD, 
                        0xDE, 
                        0xDF, 
                        0xC0, 
                        0xC1, 
                        0xC2, 
                        0xC3, 
                        0xC4, 
                        0xC5, 
                        0xC6, 
                        0xC7, 
                        0xF0, 
                        0xF1, 
                        0xF2, 
                        0xF3, 
                        0xF6, 
                        0xF7, 
                        0xFA
                    })
                    Store (Match (FBDT, MEQ, Arg0, MTR, Zero, Zero), Local0)
                    Increment (Local0)
                    Return (Local0)
                }
            }

            Method (WMNB, 3, Serialized)
            {
                CreateDWordField (Arg2, Zero, IIA0)
                CreateDWordField (Arg2, 0x04, IIA1)
                CreateDWordField (Arg2, 0x08, IIA2)
                CreateDWordField (Arg2, 0x0C, IIA3)
                CreateDWordField (Arg2, 0x10, IIA4)
                And (Arg1, 0xFFFFFFFF, Local0)
                If (LEqual (Local0, 0x54494E49))
                {
                    Return (INIT (IIA0))
                }

                If (LEqual (Local0, 0x53545342))
                {
                    Return (BSTS ())
                }

                If (LEqual (Local0, 0x4E554653))
                {
                    Return (SFUN ())
                }

                If (LEqual (Local0, 0x474F4457))
                {
                    Return (WDOG (IIA0))
                }

                If (LEqual (Local0, 0x494E424B))
                {
                    Return (KBNI ())
                }

                If (LEqual (Local0, 0x47444353))
                {
                    Return (SCDG (IIA0, IIA1))
                }

                If (LEqual (Local0, 0x43455053))
                {
                    Return (SPEC (IIA0))
                }

                If (LEqual (Local0, 0x5256534F))
                {
                    OSVR (IIA0)
                    Return (Zero)
                }

                If (LEqual (Local0, 0x53524556))
                {
                    Return (VERS (IIA0, IIA1))
                }

                If (LEqual (Local0, 0x44434C47))
                {
                    Return (GLCD ())
                }

                If (LEqual (Local0, 0x49564E41))
                {
                    Return (ANVI (IIA0))
                }

                If (LEqual (Local0, 0x4647574D))
                {
                    If (LEqual (IIA0, 0x00020013)){}
                    If (LEqual (IIA0, 0x00010016))
                    {
                        Store (OFBD (IIA1), Local0)
                        If (Local0)
                        {
                            Store (One, SMIF) /* \_SB_.SMIF */
                            Return (ASMI (IIA1))
                        }

                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x0006001F))
                    {
                        Store (0x02, SMIF) /* \_SB_.SMIF */
                        Return (ASMI (Zero))
                    }

                    If (LEqual (IIA0, 0x0010001F))
                    {
                        Store (0x03, SMIF) /* \_SB_.SMIF */
                        Return (ASMI (IIA1))
                    }
                }

                If (LEqual (Local0, 0x53545344))
                {
                    If (LEqual (IIA0, 0x00010002))
                    {
                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00010011))
                    {
                        If (WLDP)
                        {
                            Return (0x00030001)
                        }
                        Else
                        {
                            Return (0x02)
                        }
                    }

                    If (LEqual (IIA0, 0x00010013))
                    {
                        If (BTDP)
                        {
                            Return (0x00030001)
                        }
                        Else
                        {
                            Return (0x02)
                        }
                    }

                    If (LEqual (IIA0, 0x00080041))
                    {
                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00080042))
                    {
                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00080043))
                    {
                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00080044))
                    {
                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00110024))
                    {
                        If (LEqual (IIA1, Zero))
                        {
                            Store (0x80, Local0)
                        }
                        ElseIf (LEqual (IIA1, One))
                        {
                            Store (0x81, Local0)
                        }
                        ElseIf (LEqual (IIA1, 0x02))
                        {
                            Store (0x82, Local0)
                        }
                        Else
                        {
                            Return (Zero)
                        }

                        Return (^^PCI0.SBRG.EC0.GDFC (Local0))
                    }

                    If (LEqual (IIA0, 0x00110025))
                    {
                        If (LEqual (IIA1, Zero))
                        {
                            Store (0x84, Local0)
                        }
                        ElseIf (LEqual (IIA1, One))
                        {
                            Store (0x85, Local0)
                        }
                        ElseIf (LEqual (IIA1, 0x02))
                        {
                            Store (0x86, Local0)
                        }
                        Else
                        {
                            Return (Zero)
                        }

                        Return (^^PCI0.SBRG.EC0.GDFC (Local0))
                    }

                    If (LEqual (IIA0, 0x00110026))
                    {
                        Return (^^PCI0.SBRG.EC0.GFLB (0x20))
                    }

                    If (LEqual (IIA0, 0x00110027))
                    {
                        Return (^^PCI0.SBRG.EC0.GFLB (0x24))
                    }

                    If (LEqual (IIA0, 0x00060061))
                    {
                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00130022))
                    {
                        If (APSC)
                        {
                            Return (0x00010001)
                        }
                        Else
                        {
                            Return (0x00010000)
                        }
                    }

                    If (LEqual (IIA0, 0x00130021))
                    {
                        Return (0x00010002)
                    }

                    If (LEqual (IIA0, 0x00110013))
                    {
                        Store (^^PCI0.SBRG.EC0.RTAH (Zero), Local0)
                        Divide (Local0, 0x64, , Local0)
                        Return (Add (0x00010000, Local0))
                    }

                    If (LEqual (IIA0, 0x00110014))
                    {
                        Store (^^PCI0.SBRG.EC0.RTAH (One), Local0)
                        Divide (Local0, 0x64, , Local0)
                        Return (Add (0x00010000, Local0))
                    }

                    If (LEqual (IIA0, 0x00010001))
                    {
                        Return (0x00040000)
                    }

                    If (LEqual (IIA0, 0x00120061))
                    {
                        If (LEqual (^^PCI0.SBRG.EC0.ACNG, One))
                        {
                            Return (0x00010002)
                        }

                        If (^^PCI0.SBRG.EC0.ACPS ())
                        {
                            Return (0x00010001)
                        }
                        Else
                        {
                            Return (0x00010000)
                        }
                    }

                    If (LEqual (IIA0, 0x00110015))
                    {
                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00110016))
                    {
                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00050021))
                    {
                        Return (0xFFFFFFFE)
                    }

                    If (LEqual (IIA0, 0x00100051))
                    {
                        Return (0x00010001)
                    }

                    If (LEqual (IIA0, 0x00120057))
                    {
                        Name (RBU1, Buffer (0x02)
                        {
                             0x00, 0x00                                       // ..
                        })
                        Store (^^PCI0.SBRG.EC0.REBC (0x05, 0x02), RBU1) /* \_SB_.ATKD.WMNB.RBU1 */
                        Store (DerefOf (Index (RBU1, One)), Local1)
                        Store (0x00010000, Local0)
                        Or (Local0, 0x00080000, Local0)
                        If (And (Local1, One))
                        {
                            Or (Local0, 0x00100000, Local0)
                        }

                        If (And (Local1, 0x80))
                        {
                            Or (Local0, 0x0200, Local0)
                        }

                        Return (Local0)
                    }

                    If (LEqual (IIA0, 0x00110022))
                    {
                        Store (^^PCI0.SBRG.EC0.STCC (Zero, 0xD3), Local0)
                        Or (Local0, 0x00010000, Local0)
                        Return (Local0)
                    }

                    If (LEqual (IIA0, 0x00110023))
                    {
                        Store (^^PCI0.SBRG.EC0.STCC (Zero, 0xDB), Local0)
                        Or (Local0, 0x00010000, Local0)
                        Return (Local0)
                    }

                    If (LEqual (IIA0, 0x00120075))
                    {
                        Return (0x00010003)
                    }

                    If (LEqual (IIA0, 0x00060023))
                    {
                        Store (And (^^PCI0.SBRG.EC0.STA8 (Zero), 0x03), Local0)
                        If (Ones)
                        {
                            Or (0x04, Local0, Local0)
                        }

                        If (LEqual (And (^^PCI0.SBRG.EC0.STA8 (Zero), 0x80), 0x80))
                        {
                            If (LEqual (^^PCI0.SBRG.EC0.STA8 (0x02), 0x03))
                            {
                                Or (0x08, Local0, Local0)
                            }
                        }

                        Return (Local0)
                    }

                    If (LEqual (IIA0, 0x00060024))
                    {
                        Return (Package (0x03)
                        {
                            0xA36D8086, 
                            One, 
                            0xFFFFFFFF
                        })
                    }

                    If (LEqual (IIA0, 0x00060026))
                    {
                        Store (^^PCI0.SBRG.EC0.STA8 (Zero), Local0)
                        And (Local0, 0x04, Local0)
                        If (LEqual (Local0, 0x04))
                        {
                            Return (0x00010001)
                        }
                        ElseIf (LEqual (Local0, Zero))
                        {
                            Return (0x00010000)
                        }
                    }

                    If (LEqual (IIA0, 0x0012006C))
                    {
                        Name (PDPF, Zero)
                        Acquire (^^PCI0.SBRG.EC0.CMUT, 0xFFFF)
                        Store (^^PCI0.SBRG.EC0.PDST, PDPF) /* \_SB_.ATKD.WMNB.PDPF */
                        Release (^^PCI0.SBRG.EC0.CMUT)
                        Store (0x00010000, Local0)
                        If (LNotEqual (PDPF, Zero))
                        {
                            Add (Local0, PDPF, Local0)
                        }
                        Else
                        {
                            Store (Zero, Local0)
                        }

                        Return (Local0)
                    }
                }

                If (LEqual (Local0, 0x53564544))
                {
                    If (LEqual (IIA0, 0x00010002))
                    {
                        SWBL (IIA1)
                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00010012))
                    {
                        WLED (IIA1)
                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00010013))
                    {
                        BLED (IIA1)
                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00010003))
                    {
                        Return (CWAP (IIA1))
                    }

                    If (LEqual (IIA0, 0x00110024))
                    {
                        Return (^^PCI0.SBRG.EC0.SUFC (IIA1, IIA2, IIA3, IIA4, 0x40))
                    }

                    If (LEqual (IIA0, 0x00110025))
                    {
                        Return (^^PCI0.SBRG.EC0.SUFC (IIA1, IIA2, IIA3, IIA4, 0x44))
                    }

                    If (LEqual (IIA0, 0x00130022))
                    {
                        Store (0x05, SMIF) /* \_SB_.SMIF */
                        If (LEqual (IIA1, One))
                        {
                            Store (One, APSC) /* \_SB_.APSC */
                        }
                        Else
                        {
                            Store (Zero, APSC) /* \_SB_.APSC */
                        }

                        ASMI (Zero)
                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00050011))
                    {
                        If (LEqual (IIA1, 0x02))
                        {
                            Store (One, ^^PCI0.SBRG.EC0.BLCT) /* \_SB_.PCI0.SBRG.EC0_.BLCT */
                        }

                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00050021))
                    {
                        ^^PCI0.SBRG.EC0.SLKB (IIA1)
                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00100022))
                    {
                        If (And (IIA1, 0x02))
                        {
                            ^^PCI0.SBRG.EC0.STB1 (0x04)
                            ^^PCI0.SBRG.EC0.STB1 (0x05)
                            Store (One, FNIV) /* \_SB_.FNIV */
                            Return (One)
                        }
                        Else
                        {
                            ^^PCI0.SBRG.EC0.KINI ()
                            Return (One)
                        }

                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00100021))
                    {
                        If (LEqual (IIA1, 0x6C))
                        {
                            ^^PCI0.SBRG.EC0._Q0A ()
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0x88))
                        {
                            ^^PCI0.SBRG.EC0._Q0B ()
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0xC5))
                        {
                            ^^PCI0.SBRG.EC0.KBLD ()
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0xC4))
                        {
                            ^^PCI0.SBRG.EC0.KBLU ()
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0x10))
                        {
                            ^^PCI0.SBRG.EC0._Q0E ()
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0x20))
                        {
                            ^^PCI0.SBRG.EC0._Q0F ()
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0x35))
                        {
                            ^^PCI0.SBRG.EC0._Q10 ()
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0x6B))
                        {
                            ^^PCI0.SBRG.EC0._Q12 ()
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0x8A))
                        {
                            ^^PCI0.SBRG.EC0._Q72 ()
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0x38))
                        {
                            ^^PCI0.SBRG.EC0._Q6B ()
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0xAE))
                        {
                            IANE (IIA1)
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0x7C))
                        {
                            IANE (IIA1)
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0x9E))
                        {
                            IANE (IIA1)
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0xA8))
                        {
                            IANE (IIA1)
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0xA9))
                        {
                            IANE (IIA1)
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0xAA))
                        {
                            IANE (IIA1)
                            Return (Zero)
                        }

                        If (LEqual (IIA1, 0xAB))
                        {
                            IANE (IIA1)
                            Return (Zero)
                        }

                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00110013))
                    {
                        Store (^^PCI0.SBRG.EC0.RRAM (0xCC, 0x30), Local0)
                        If (LEqual (IIA1, Zero))
                        {
                            And (Local0, 0xFFFFFFFFFFFFFFBF, Local1)
                        }
                        ElseIf (LEqual (IIA1, One))
                        {
                            Or (Local0, 0x40, Local1)
                        }

                        ^^PCI0.SBRG.EC0.WRAM (0xCD, 0x30, Local1)
                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00110014))
                    {
                        Store (^^PCI0.SBRG.EC0.RRAM (0xCC, 0x10), Local0)
                        If (And (Local0, 0x02))
                        {
                            Store (^^PCI0.SBRG.EC0.RRAM (0xCC, 0x30), Local0)
                            If (LEqual (IIA1, Zero))
                            {
                                And (Local0, 0xFFFFFFFFFFFFFFBF, Local1)
                            }
                            ElseIf (LEqual (IIA1, One))
                            {
                                Or (Local0, 0x40, Local1)
                            }

                            ^^PCI0.SBRG.EC0.WRAM (0xCD, 0x30, Local1)
                            Return (One)
                        }

                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00110015))
                    {
                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00110016))
                    {
                        Return (Zero)
                    }

                    If (LEqual (IIA0, 0x00120057))
                    {
                        Return (^^PCI0.SBRG.EC0.SRSC (IIA1))
                    }

                    If (LEqual (IIA0, 0x00110022))
                    {
                        Store (IIA1, Local2)
                        And (Local2, 0xFF, Local0)
                        And (Local2, 0xFF00, Local1)
                        ShiftRight (Local1, 0x08, Local1)
                        ^^PCI0.SBRG.EC0.STCD (Zero, 0xD2, Local0)
                        ^^PCI0.SBRG.EC0.STCD (Zero, 0xD1, Local1)
                        Store (^^PCI0.SBRG.EC0.STCC (Zero, 0xD0), Local3)
                        Or (Local3, One, Local3)
                        ^^PCI0.SBRG.EC0.STCD (Zero, 0xD0, Local3)
                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00110023))
                    {
                        Store (IIA1, Local2)
                        And (Local2, 0xFF, Local0)
                        And (Local2, 0xFF00, Local1)
                        ShiftRight (Local1, 0x08, Local1)
                        ^^PCI0.SBRG.EC0.STCD (Zero, 0xDA, Local0)
                        ^^PCI0.SBRG.EC0.STCD (Zero, 0xD9, Local1)
                        Store (^^PCI0.SBRG.EC0.STCC (Zero, 0xD8), Local3)
                        Or (Local3, One, Local3)
                        ^^PCI0.SBRG.EC0.STCD (Zero, 0xD8, Local3)
                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00120075))
                    {
                        ^^PCI0.SBRG.EC0.WEBC (0x23, Zero, Zero)
                        Store (^^PCI0.SBRG.EC0.STCC (Zero, 0xD0), Local3)
                        And (Local3, 0xFE, Local3)
                        ^^PCI0.SBRG.EC0.STCD (Zero, 0xD0, Local3)
                        Store (^^PCI0.SBRG.EC0.STCC (Zero, 0xD8), Local3)
                        And (Local3, 0xFE, Local3)
                        ^^PCI0.SBRG.EC0.STCD (Zero, 0xD8, Local3)
                        If (LEqual (IIA1, One))
                        {
                            ^^PCI0.SBRG.EC0.STCD (Zero, 0x13, 0x04)
                        }
                        ElseIf (LEqual (IIA1, Zero))
                        {
                            ^^PCI0.SBRG.EC0.STCD (Zero, 0x13, One)
                        }
                        ElseIf (LEqual (IIA1, 0x02))
                        {
                            ^^PCI0.SBRG.EC0.STCD (Zero, 0x13, 0x02)
                        }

                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00130025))
                    {
                        If (LEqual (IIA1, 0x02))
                        {
                            ^^PCI0.SBRG.EC0.STE7 (Zero)
                        }

                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00060023))
                    {
                        ShiftRight (IIA1, 0x09, Local0)
                        If (LEqual (And (Local0, One), One))
                        {
                            Store (One, VBOF) /* \_SB_.VBOF */
                            ShiftRight (IIA1, 0x18, Local0)
                            Multiply (Local0, 0x0100, Local0)
                            Or (Local0, VBOF, VBOF) /* \_SB_.VBOF */
                        }
                        Else
                        {
                            Store (Zero, VBOF) /* \_SB_.VBOF */
                        }

                        And (IIA1, 0xFF, Local0)
                        ^^PCI0.SBRG.EC0.STA9 (One, Local0)
                        ShiftRight (IIA1, 0x08, Local0)
                        Store (^^PCI0.SBRG.EC0.STA8 (Zero), Local1)
                        If (LEqual (And (Local0, One), One))
                        {
                            Or (Local1, 0x02, Local1)
                            And (Local1, 0x0F, Local2)
                            ^^PCI0.SBRG.EC0.STA9 (Zero, Local1)
                        }
                        Else
                        {
                            And (Local1, 0xFD, Local1)
                            And (Local1, 0x0F, Local2)
                            ^^PCI0.SBRG.EC0.STA9 (Zero, Local1)
                        }

                        Return (One)
                    }

                    If (LEqual (IIA0, 0x00060026))
                    {
                        Store (^^PCI0.SBRG.EC0.STA8 (Zero), Local0)
                        If (LEqual (IIA1, One))
                        {
                            Or (Local0, 0x04, Local0)
                            ^^PCI0.SBRG.EC0.STA9 (Zero, Local0)
                        }
                        Else
                        {
                            And (Local0, 0xFB, Local0)
                            ^^PCI0.SBRG.EC0.STA9 (Zero, Local0)
                        }

                        Return (One)
                    }
                }

                Return (0xFFFFFFFE)
            }

            Method (_WED, 1, NotSerialized)  // _Wxx: Wake Event, xx=0x00-0xFF
            {
                If (LEqual (Arg0, 0xFF))
                {
                    Return (GANQ ())
                }

                Return (Ones)
            }

            Name (WQMO, Buffer (0x08F9)
            {
                /* 0000 */  0x46, 0x4F, 0x4D, 0x42, 0x01, 0x00, 0x00, 0x00,  // FOMB....
                /* 0008 */  0xE9, 0x08, 0x00, 0x00, 0x10, 0x2B, 0x00, 0x00,  // .....+..
                /* 0010 */  0x44, 0x53, 0x00, 0x01, 0x1A, 0x7D, 0xDA, 0x54,  // DS...}.T
                /* 0018 */  0x28, 0xC5, 0x94, 0x00, 0x01, 0x06, 0x18, 0x42,  // (......B
                /* 0020 */  0x10, 0x05, 0x10, 0x0A, 0x60, 0x89, 0x42, 0x04,  // ....`.B.
                /* 0028 */  0x0A, 0x0D, 0xA1, 0x38, 0x44, 0x86, 0xA1, 0x12,  // ...8D...
                /* 0030 */  0x20, 0x24, 0x09, 0x42, 0x2E, 0x98, 0x98, 0x00,  //  $.B....
                /* 0038 */  0x21, 0x10, 0x92, 0x28, 0xC0, 0xBC, 0x00, 0xDD,  // !..(....
                /* 0040 */  0x02, 0x0C, 0x0B, 0xB0, 0x2D, 0xC0, 0xB4, 0x00,  // ....-...
                /* 0048 */  0xC7, 0x40, 0xEA, 0xDF, 0x1F, 0xA2, 0x34, 0x10,  // .@....4.
                /* 0050 */  0x89, 0x80, 0xA4, 0x52, 0x20, 0x24, 0x54, 0x80,  // ...R $T.
                /* 0058 */  0x72, 0x01, 0xBE, 0x05, 0x68, 0x47, 0x94, 0x64,  // r...hG.d
                /* 0060 */  0x01, 0x96, 0x61, 0x44, 0x60, 0xAF, 0xC8, 0x04,  // ..aD`...
                /* 0068 */  0x8D, 0x13, 0x94, 0x33, 0x0C, 0x14, 0xBC, 0x01,  // ...3....
                /* 0070 */  0xDB, 0x14, 0x60, 0x72, 0x10, 0x54, 0xF6, 0x20,  // ..`r.T. 
                /* 0078 */  0x50, 0x32, 0x20, 0xE4, 0x51, 0x80, 0x55, 0x38,  // P2 .Q.U8
                /* 0080 */  0x4D, 0x27, 0x81, 0xDD, 0x0B, 0x30, 0x27, 0xC0,  // M'...0'.
                /* 0088 */  0x9B, 0x00, 0x71, 0xA3, 0x91, 0x35, 0x01, 0x16,  // ..q..5..
                /* 0090 */  0xA1, 0x24, 0x4C, 0x80, 0x2D, 0x18, 0xA1, 0xD4,  // .$L.-...
                /* 0098 */  0x06, 0x23, 0x94, 0xC6, 0x10, 0x04, 0x12, 0x27,  // .#.....'
                /* 00A0 */  0x4A, 0xC4, 0x06, 0x05, 0x38, 0x83, 0x15, 0x4A,  // J...8..J
                /* 00A8 */  0xC4, 0x30, 0x21, 0x22, 0x54, 0x86, 0x41, 0x1C,  // .0!"T.A.
                /* 00B0 */  0x42, 0x09, 0x2D, 0x6E, 0x84, 0xF6, 0x07, 0x41,  // B.-n...A
                /* 00B8 */  0xA2, 0xAD, 0x58, 0x43, 0x75, 0xA4, 0xD1, 0xA0,  // ..XCu...
                /* 00C0 */  0x46, 0x94, 0xE0, 0xC0, 0x3C, 0x9A, 0x53, 0xED,  // F...<.S.
                /* 00C8 */  0x5C, 0x80, 0xF4, 0xB9, 0x0A, 0xE4, 0x58, 0x4F,  // \.....XO
                /* 00D0 */  0xB1, 0xCE, 0xE1, 0x12, 0x90, 0x04, 0xC6, 0x4A,  // .......J
                /* 00D8 */  0xD0, 0xC1, 0x10, 0xA1, 0xB8, 0x06, 0xD4, 0xC4,  // ........
                /* 00E0 */  0x0F, 0x91, 0x09, 0x82, 0x43, 0x0D, 0xD1, 0xE3,  // ....C...
                /* 00E8 */  0x0D, 0x77, 0x02, 0x47, 0x7D, 0x24, 0x0C, 0xE2,  // .w.G}$..
                /* 00F0 */  0x9C, 0x8E, 0x06, 0x73, 0x0E, 0xF0, 0xB8, 0x4E,  // ...s...N
                /* 00F8 */  0xE6, 0x8C, 0x4B, 0x15, 0x60, 0x76, 0xE8, 0x1A,  // ..K.`v..
                /* 0100 */  0x61, 0x82, 0xE3, 0x31, 0xF4, 0xD9, 0x9F, 0x4F,  // a..1...O
                /* 0108 */  0xCF, 0x90, 0x44, 0xFE, 0x20, 0x50, 0x23, 0x33,  // ..D. P#3
                /* 0110 */  0xB4, 0xC7, 0x78, 0x5A, 0x31, 0x43, 0x3E, 0x05,  // ..xZ1C>.
                /* 0118 */  0x1C, 0x16, 0x13, 0x7B, 0x4A, 0xA0, 0xE3, 0x81,  // ...{J...
                /* 0120 */  0xF7, 0xFF, 0x1F, 0x0F, 0x78, 0x14, 0x1F, 0x0D,  // ....x...
                /* 0128 */  0x84, 0xF0, 0x4A, 0x10, 0xDB, 0x03, 0x7A, 0x5C,  // ..J...z\
                /* 0130 */  0x30, 0x30, 0x76, 0x40, 0xF6, 0x2B, 0x00, 0x21,  // 00v@.+.!
                /* 0138 */  0x78, 0x99, 0x23, 0x92, 0x53, 0x04, 0x8D, 0xC9,  // x.#.S...
                /* 0140 */  0x93, 0x6B, 0x1D, 0x42, 0x20, 0x27, 0x75, 0xC8,  // .k.B 'u.
                /* 0148 */  0x2F, 0x0B, 0x0C, 0xF1, 0x35, 0xC1, 0xD3, 0xAF,  // /...5...
                /* 0150 */  0x10, 0x50, 0x22, 0x47, 0x80, 0x12, 0x33, 0x02,  // .P"G..3.
                /* 0158 */  0x14, 0x5E, 0xE4, 0x90, 0x67, 0x73, 0x04, 0xC7,  // .^..gs..
                /* 0160 */  0x13, 0xE5, 0x24, 0x0E, 0xC8, 0x47, 0x07, 0x23,  // ..$..G.#
                /* 0168 */  0x1C, 0x42, 0xB9, 0x27, 0x08, 0xA2, 0x79, 0x46,  // .B.'..yF
                /* 0170 */  0x9A, 0xC6, 0x39, 0xBD, 0x0D, 0x98, 0x60, 0x50,  // ..9...`P
                /* 0178 */  0x45, 0x10, 0x75, 0x40, 0x7A, 0xA9, 0xB2, 0xB1,  // E.u@z...
                /* 0180 */  0x54, 0x8F, 0x94, 0xE6, 0x3F, 0x23, 0x68, 0x86,  // T...?#h.
                /* 0188 */  0xE7, 0x10, 0xE2, 0x10, 0x13, 0x38, 0x39, 0x90,  // .....89.
                /* 0190 */  0x4E, 0x1A, 0x3C, 0x33, 0x89, 0x52, 0x8F, 0x86,  // N.<3.R..
                /* 0198 */  0xC2, 0x39, 0xE7, 0x81, 0x83, 0x82, 0x18, 0xD0,  // .9......
                /* 01A0 */  0x21, 0x20, 0xE4, 0xE4, 0x1C, 0x80, 0x1A, 0x84,  // ! ......
                /* 01A8 */  0x07, 0xF5, 0xBC, 0x60, 0x88, 0xD7, 0x00, 0x8B,  // ...`....
                /* 01B0 */  0x3E, 0x2C, 0xD0, 0xE1, 0xF9, 0x18, 0xC0, 0x35,  // >,.....5
                /* 01B8 */  0x40, 0xE8, 0x5E, 0x60, 0x64, 0xAB, 0x01, 0xA4,  // @.^`d...
                /* 01C0 */  0x60, 0xFC, 0x70, 0xC1, 0x09, 0xEA, 0xBA, 0x0A,  // `.p.....
                /* 01C8 */  0x40, 0x1F, 0xE6, 0x11, 0x78, 0x26, 0xEF, 0x16,  // @...x&..
                /* 01D0 */  0x2F, 0x16, 0x09, 0xEA, 0x3B, 0x0A, 0x80, 0x02,  // /...;...
                /* 01D8 */  0xC8, 0x53, 0xB7, 0xD2, 0xE1, 0xD3, 0x31, 0x84,  // .S....1.
                /* 01E0 */  0x08, 0x13, 0xCD, 0xE8, 0x3C, 0xFC, 0x54, 0x51,  // ....<.TQ
                /* 01E8 */  0x71, 0xA7, 0x4A, 0x41, 0x3C, 0xD5, 0xFF, 0x7F,  // q.JA<...
                /* 01F0 */  0x82, 0x51, 0xA6, 0x8A, 0x9E, 0x89, 0xA7, 0xCA,  // .Q......
                /* 01F8 */  0xEF, 0x07, 0x26, 0xB0, 0xF0, 0x7B, 0x06, 0xB4,  // ..&..{..
                /* 0200 */  0x4B, 0x41, 0xC4, 0x27, 0x02, 0x4F, 0xC3, 0x70,  // KA.'.O.p
                /* 0208 */  0x9E, 0x2C, 0x87, 0xF3, 0x64, 0xF9, 0x58, 0x7C,  // .,..d.X|
                /* 0210 */  0x8F, 0x80, 0x3F, 0x5A, 0x2C, 0x41, 0x81, 0xB3,  // ..?Z,A..
                /* 0218 */  0x05, 0x39, 0x3C, 0x46, 0xF0, 0x48, 0xA9, 0xAC,  // .9<F.H..
                /* 0220 */  0x71, 0xA1, 0x2E, 0x07, 0x3E, 0x53, 0x30, 0xEC,  // q...>S0.
                /* 0228 */  0x57, 0x84, 0xA3, 0x79, 0x45, 0x38, 0xC3, 0xA7,  // W..yE8..
                /* 0230 */  0x88, 0x83, 0x7A, 0xE6, 0xB0, 0x49, 0x9D, 0x26,  // ..z..I.&
                /* 0238 */  0xA5, 0xC2, 0x7A, 0xA4, 0x1C, 0xD6, 0x68, 0x61,  // ..z...ha
                /* 0240 */  0x0F, 0xF8, 0x75, 0xC3, 0x67, 0x15, 0xCF, 0xCC,  // ..u.g...
                /* 0248 */  0x18, 0x61, 0x3D, 0x5A, 0x07, 0x5A, 0xA7, 0x2E,  // .a=Z.Z..
                /* 0250 */  0x13, 0x38, 0x59, 0x40, 0xBA, 0x66, 0x1C, 0xD1,  // .8Y@.f..
                /* 0258 */  0x31, 0x1D, 0x12, 0x13, 0x74, 0x99, 0xA0, 0x10,  // 1...t...
                /* 0260 */  0x96, 0x42, 0x21, 0x21, 0x10, 0x3A, 0x12, 0xF0,  // .B!!.:..
                /* 0268 */  0xE8, 0x60, 0x14, 0xC4, 0x23, 0x73, 0xAC, 0x23,  // .`..#s.#
                /* 0270 */  0x01, 0x5A, 0xD4, 0x1D, 0x82, 0x5E, 0x66, 0x70,  // .Z...^fp
                /* 0278 */  0x93, 0x7D, 0x88, 0x61, 0x20, 0xB1, 0xD8, 0xC5,  // .}.a ...
                /* 0280 */  0x80, 0x1F, 0x0C, 0xC0, 0xFC, 0xFF, 0x27, 0x60,  // ......'`
                /* 0288 */  0x07, 0x03, 0xC0, 0x93, 0x42, 0xA7, 0x52, 0x9D,  // ....B.R.
                /* 0290 */  0x1E, 0x0C, 0xC0, 0x15, 0xF2, 0x60, 0x80, 0x96,  // .....`..
                /* 0298 */  0x0A, 0xA4, 0x09, 0xD5, 0x78, 0x80, 0xD2, 0xAD,  // ....x...
                /* 02A0 */  0x25, 0x81, 0x45, 0x1E, 0x0C, 0x50, 0xF2, 0x28,  // %.E..P.(
                /* 02A8 */  0x24, 0xEE, 0x60, 0x80, 0x92, 0x03, 0x47, 0x41,  // $.`...GA
                /* 02B0 */  0x3C, 0x32, 0x47, 0x3D, 0x18, 0xA0, 0xC7, 0x99,  // <2G=....
                /* 02B8 */  0xE0, 0x20, 0x8F, 0xE5, 0x78, 0x0E, 0xEC, 0xBD,  // . ..x...
                /* 02C0 */  0xE6, 0x6C, 0x9E, 0x0A, 0x3C, 0xF4, 0x38, 0x81,  // .l..<.8.
                /* 02C8 */  0xF8, 0x8C, 0x0C, 0xF5, 0x6A, 0x13, 0xEA, 0xF9,  // ....j...
                /* 02D0 */  0xC0, 0xA7, 0x22, 0x83, 0xBD, 0x23, 0x18, 0xEA,  // .."..#..
                /* 02D8 */  0xF1, 0x20, 0xA6, 0x27, 0xF7, 0xF2, 0xE5, 0x13,  // . .'....
                /* 02E0 */  0x82, 0xD1, 0xF8, 0x11, 0x01, 0x38, 0xFF, 0xFF,  // .....8..
                /* 02E8 */  0x8F, 0x08, 0xC0, 0xDA, 0xA9, 0x46, 0xAB, 0x3A,  // .....F.:
                /* 02F0 */  0x8F, 0x08, 0xE0, 0x8A, 0x74, 0x4D, 0x42, 0x0B,  // ....tMB.
                /* 02F8 */  0x03, 0xD2, 0x38, 0x0E, 0xB7, 0xCF, 0x33, 0x9B,  // ..8...3.
                /* 0300 */  0x08, 0x2C, 0x09, 0x48, 0xD7, 0x24, 0x2E, 0x86,  // .,.H.$..
                /* 0308 */  0x42, 0x32, 0x8E, 0x08, 0xA8, 0xE0, 0x07, 0x05,  // B2......
                /* 0310 */  0x0A, 0x62, 0x40, 0xA7, 0x3A, 0x22, 0xA0, 0x0F,  // .b@.:"..
                /* 0318 */  0x45, 0xF6, 0x7E, 0x24, 0x20, 0x77, 0x82, 0x23,  // E.~$ w.#
                /* 0320 */  0x3B, 0x99, 0x33, 0x7B, 0x0F, 0x3B, 0xD9, 0xB7,  // ;.3{.;..
                /* 0328 */  0x02, 0x0F, 0xC9, 0x57, 0x84, 0x30, 0xC1, 0x1E,  // ...W.0..
                /* 0330 */  0x0C, 0x7C, 0x1E, 0xF3, 0x85, 0xCB, 0xE7, 0x25,  // .|.....%
                /* 0338 */  0xC0, 0x97, 0x88, 0x31, 0xA3, 0x62, 0x8F, 0x99,  // ...1.b..
                /* 0340 */  0x82, 0x78, 0xCC, 0xBE, 0xE9, 0xFD, 0xFF, 0x8F,  // .x......
                /* 0348 */  0x19, 0xFB, 0x11, 0xF0, 0x74, 0x71, 0x57, 0x0C,  // ....tqW.
                /* 0350 */  0x9F, 0x65, 0x80, 0xCB, 0x99, 0x09, 0x70, 0xE5,  // .e....p.
                /* 0358 */  0x55, 0xA2, 0x4F, 0x8F, 0x27, 0x0B, 0x70, 0x85,  // U.O.'.p.
                /* 0360 */  0x3B, 0xCB, 0xA0, 0x25, 0x02, 0xC9, 0xE5, 0x9D,  // ;..%....
                /* 0368 */  0x49, 0x07, 0x3A, 0x13, 0xF8, 0x1C, 0xE3, 0xB3,  // I.:.....
                /* 0370 */  0x0C, 0x3F, 0x3A, 0x59, 0xD4, 0xB1, 0x00, 0x75,  // .?:Y...u
                /* 0378 */  0xB8, 0x30, 0x98, 0x41, 0x3C, 0x32, 0x47, 0x3C,  // .0.A<2G<
                /* 0380 */  0x16, 0xA0, 0x47, 0xEC, 0xF3, 0xC0, 0x63, 0x81,  // ..G...c.
                /* 0388 */  0x6F, 0x7B, 0x3E, 0xD1, 0xB1, 0x7B, 0x8C, 0xEF,  // o{>..{..
                /* 0390 */  0x4C, 0xEC, 0x5C, 0x68, 0xA8, 0x57, 0x83, 0x48,  // L.\h.W.H
                /* 0398 */  0x1E, 0xD5, 0x9B, 0x9F, 0x51, 0x1F, 0x67, 0xDE,  // ....Q.g.
                /* 03A0 */  0x63, 0xB0, 0xC7, 0x03, 0x78, 0xFF, 0xFF, 0x73,  // c...x..s
                /* 03A8 */  0x0C, 0x60, 0xD4, 0xA5, 0xC2, 0x17, 0xDA, 0xE3,  // .`......
                /* 03B0 */  0x01, 0xB8, 0x44, 0xAD, 0x42, 0xC7, 0x03, 0x5C,  // ..D.B..\
                /* 03B8 */  0xB4, 0xB5, 0x68, 0x42, 0x67, 0x76, 0xBA, 0xCF,  // ..hBgv..
                /* 03C0 */  0x31, 0x2C, 0xD4, 0x39, 0x06, 0x75, 0xE7, 0x75,  // 1,.9.u.u
                /* 03C8 */  0xA4, 0xE3, 0x01, 0x2A, 0xC4, 0xF1, 0x80, 0x82,  // ...*....
                /* 03D0 */  0x78, 0x5C, 0x0E, 0x77, 0x3C, 0x40, 0xCB, 0xBB,  // x\.w<@..
                /* 03D8 */  0xDB, 0x92, 0x89, 0xF9, 0x06, 0xE2, 0x09, 0x45,  // .......E
                /* 03E0 */  0x79, 0x2A, 0x78, 0x2E, 0xE0, 0xC7, 0x18, 0x1F,  // y*x.....
                /* 03E8 */  0x0C, 0xDE, 0x98, 0x7C, 0x40, 0x78, 0x3E, 0x78,  // ...|@x>x
                /* 03F0 */  0x35, 0xC0, 0x80, 0x9D, 0x56, 0x1C, 0x43, 0xBC,  // 5...V.C.
                /* 03F8 */  0xCA, 0x00, 0xC7, 0xFF, 0xFF, 0xDD, 0x80, 0xA5,  // ........
                /* 0400 */  0x18, 0x1E, 0xFA, 0x0E, 0xEA, 0x31, 0x1D, 0x5A,  // .....1.Z
                /* 0408 */  0x84, 0xB3, 0x3B, 0x3F, 0x4F, 0x09, 0xBC, 0xE3,  // ..;?O...
                /* 0410 */  0xF6, 0x69, 0x07, 0x38, 0x9F, 0x02, 0x7D, 0x32,  // .i.8..}2
                /* 0418 */  0xE0, 0x61, 0x87, 0x44, 0x41, 0x7C, 0x42, 0x70,  // .a.DA|Bp
                /* 0420 */  0x90, 0x6B, 0x2A, 0x94, 0xCB, 0x00, 0xE6, 0xE8,  // .k*.....
                /* 0428 */  0x01, 0xF6, 0x81, 0xF8, 0x44, 0x06, 0x88, 0x72,  // ....D..r
                /* 0430 */  0x28, 0xF1, 0xA0, 0xA3, 0xFF, 0xFF, 0xD1, 0x03,  // (.......
                /* 0438 */  0x4C, 0x99, 0x4E, 0x64, 0x68, 0x65, 0x47, 0x0F,  // L.NdheG.
                /* 0440 */  0x32, 0x4A, 0x4F, 0x2D, 0x81, 0x35, 0x1D, 0x3D,  // 2JO-.5.=
                /* 0448 */  0x50, 0x6A, 0x28, 0xA4, 0xE7, 0xE8, 0x81, 0x52,  // Pj(....R
                /* 0450 */  0x70, 0x1D, 0xE6, 0x07, 0x1D, 0x1A, 0xEF, 0xA0,  // p.......
                /* 0458 */  0x03, 0xE5, 0xE8, 0x01, 0xEF, 0xDA, 0x81, 0x3B,  // .......;
                /* 0460 */  0x54, 0xB1, 0x71, 0x9D, 0xA1, 0x21, 0x0F, 0xED,  // T.q..!..
                /* 0468 */  0xD1, 0xCC, 0x07, 0x9E, 0xC0, 0xBE, 0x14, 0x03,  // ........
                /* 0470 */  0x7E, 0x0F, 0xE7, 0x3E, 0x48, 0xFA, 0xB4, 0xCD,  // ~..>H...
                /* 0478 */  0x6F, 0x64, 0xC0, 0xE3, 0xFF, 0x7F, 0xB3, 0x00,  // od......
                /* 0480 */  0x04, 0x09, 0xB5, 0xE8, 0xF0, 0xEA, 0x08, 0xFE,  // ........
                /* 0488 */  0x9B, 0xE3, 0x11, 0xD5, 0x3A, 0x25, 0x5D, 0x1D,  // ....:%].
                /* 0490 */  0xE1, 0xDC, 0x53, 0x7D, 0x24, 0xE0, 0x64, 0x86,  // ..S}$.d.
                /* 0498 */  0x33, 0x85, 0x01, 0x9D, 0xEE, 0x48, 0x80, 0xBE,  // 3....H..
                /* 04A0 */  0x2E, 0x78, 0x3C, 0x07, 0xF3, 0x58, 0xF1, 0x48,  // .x<..X.H
                /* 04A8 */  0xE1, 0xF1, 0x9C, 0xEB, 0x7B, 0x5A, 0xA0, 0xF3,  // ....{Z..
                /* 04B0 */  0xF0, 0x19, 0xC9, 0x93, 0x0F, 0xF5, 0x66, 0xE0,  // ......f.
                /* 04B8 */  0x41, 0x19, 0x28, 0x16, 0xD3, 0x7E, 0x04, 0x24,  // A.(..~.$
                /* 04C0 */  0xD7, 0x33, 0x9F, 0xC8, 0xD8, 0xC9, 0x02, 0x30,  // .3.....0
                /* 04C8 */  0xF2, 0xFF, 0xBF, 0xBD, 0x03, 0x1C, 0xF2, 0xE9,  // ........
                /* 04D0 */  0x54, 0xAB, 0xD2, 0x93, 0x05, 0xB8, 0x14, 0x5F,  // T......_
                /* 04D8 */  0xE6, 0xD0, 0xB9, 0xD7, 0xA2, 0x83, 0xC5, 0x21,  // .......!
                /* 04E0 */  0x61, 0x12, 0x9F, 0x2C, 0x50, 0x59, 0x29, 0x94,  // a..,PY).
                /* 04E8 */  0xEC, 0x64, 0x81, 0xCA, 0x72, 0xB2, 0xA0, 0x20,  // .d..r.. 
                /* 04F0 */  0x1E, 0x97, 0x6F, 0xEF, 0xF8, 0xD1, 0xFB, 0x80,  // ..o.....
                /* 04F8 */  0xFB, 0x0A, 0xC4, 0x0E, 0x8F, 0x1E, 0x11, 0x8C,  // ........
                /* 0500 */  0x5B, 0x05, 0xF6, 0xFF, 0x7F, 0xAB, 0x00, 0xAE,  // [.......
                /* 0508 */  0x77, 0x59, 0xFC, 0x45, 0xDF, 0x37, 0x82, 0x67,  // wY.E.7.g
                /* 0510 */  0x6E, 0x38, 0x23, 0x03, 0x1E, 0x07, 0x3A, 0xE0,  // n8#...:.
                /* 0518 */  0x2D, 0xE8, 0x72, 0x80, 0x92, 0x70, 0x39, 0xA0,  // -.r..p9.
                /* 0520 */  0x20, 0x3E, 0x6F, 0xC1, 0x11, 0x37, 0x43, 0x72,  //  >o..7Cr
                /* 0528 */  0x35, 0x00, 0xC7, 0x25, 0xC1, 0x18, 0xBE, 0xCD,  // 5..%....
                /* 0530 */  0xF0, 0xB3, 0x1D, 0x70, 0xF8, 0xFF, 0x5F, 0x44,  // ...p.._D
                /* 0538 */  0x00, 0x4B, 0x5A, 0x2D, 0x2A, 0x75, 0x7A, 0x11,  // .KZ-*uz.
                /* 0540 */  0x01, 0x57, 0x9C, 0x33, 0x31, 0xFA, 0x2C, 0xEC,  // .W.31.,.
                /* 0548 */  0x83, 0xDC, 0x91, 0xBD, 0xA5, 0x9A, 0xC0, 0xB7,  // ........
                /* 0550 */  0x25, 0x9F, 0x89, 0xB9, 0x10, 0x0A, 0x1D, 0xDE,  // %.......
                /* 0558 */  0xE1, 0x86, 0x3A, 0x1C, 0xA0, 0x25, 0xDD, 0xC9,  // ..:..%..
                /* 0560 */  0xE8, 0x51, 0xC7, 0x20, 0x0F, 0x05, 0x9E, 0xD1,  // .Q. ....
                /* 0568 */  0xEB, 0x45, 0x90, 0x87, 0x3A, 0x36, 0xD6, 0x13,  // .E..:6..
                /* 0570 */  0x79, 0x11, 0x60, 0x87, 0x61, 0xC0, 0xD2, 0xFF,  // y.`.a...
                /* 0578 */  0xFF, 0x5C, 0x00, 0x0C, 0x3C, 0xCA, 0x7C, 0x42,  // .\..<.|B
                /* 0580 */  0x3E, 0x17, 0x80, 0x4B, 0xFC, 0xC5, 0x0C, 0x2D,  // >..K...-
                /* 0588 */  0x01, 0x48, 0x73, 0x7A, 0xF3, 0x7E, 0xE1, 0x79,  // .Hsz.~.y
                /* 0590 */  0xF4, 0x66, 0xE1, 0xCF, 0x8E, 0xFC, 0x68, 0x6F,  // .f....ho
                /* 0598 */  0x12, 0x1D, 0x09, 0x7C, 0x2E, 0xE0, 0x51, 0xCF,  // ...|..Q.
                /* 05A0 */  0x05, 0x14, 0xC4, 0x80, 0xCE, 0x71, 0x2E, 0x40,  // .....q.@
                /* 05A8 */  0x2B, 0xB9, 0x51, 0x91, 0xC1, 0x1E, 0xE9, 0x9B,  // +.Q.....
                /* 05B0 */  0x23, 0xE6, 0xDA, 0x08, 0xB0, 0xE3, 0xFF, 0x7F,  // #.......
                /* 05B8 */  0x1B, 0x03, 0xCC, 0x19, 0xD4, 0xA9, 0xF5, 0xA8,  // ........
                /* 05C0 */  0x0E, 0x32, 0x21, 0xB7, 0x31, 0xF4, 0x19, 0xDD,  // .2!.1...
                /* 05C8 */  0xB3, 0x38, 0x8F, 0xC7, 0x96, 0x1A, 0xA1, 0x48,  // .8.....H
                /* 05D0 */  0x90, 0xDB, 0x18, 0x2A, 0x02, 0x85, 0x4E, 0xE9,  // ...*..N.
                /* 05D8 */  0x3E, 0x12, 0x70, 0xD9, 0xA7, 0x0A, 0x0A, 0xE2,  // >.p.....
                /* 05E0 */  0x71, 0xF9, 0x04, 0x8D, 0x97, 0x02, 0xA2, 0x0B,  // q.......
                /* 05E8 */  0xC0, 0xB1, 0x3C, 0x1C, 0x3D, 0x62, 0x19, 0xE4,  // ..<.=b..
                /* 05F0 */  0x5C, 0x5F, 0x02, 0xD8, 0x75, 0x02, 0x78, 0x66,  // \_..u.xf
                /* 05F8 */  0x19, 0x14, 0x7A, 0x18, 0x46, 0x7E, 0x7A, 0x32,  // ..z.F~z2
                /* 0600 */  0xE2, 0x83, 0x43, 0x90, 0xD7, 0x1A, 0xCF, 0x34,  // ..C....4
                /* 0608 */  0xC6, 0xA3, 0xCD, 0x73, 0xDB, 0x59, 0x61, 0xFF,  // ...s.Ya.
                /* 0610 */  0xFF, 0x67, 0x30, 0xC0, 0xD0, 0xA1, 0x03, 0xB8,  // .g0.....
                /* 0618 */  0x9F, 0xBC, 0x00, 0x57, 0x36, 0xBD, 0x7A, 0xD4,  // ...W6.z.
                /* 0620 */  0x78, 0xE8, 0x00, 0x57, 0x86, 0x23, 0x11, 0xF9,  // x..W.#..
                /* 0628 */  0xFF, 0x1F, 0x3A, 0x30, 0x4A, 0x80, 0x34, 0xDB,  // ..:0J.4.
                /* 0630 */  0x23, 0x3A, 0xA6, 0x43, 0x62, 0x1A, 0x0E, 0x1D,  // #:.Cb...
                /* 0638 */  0xA8, 0xF4, 0x24, 0x4A, 0x7E, 0x24, 0x40, 0x65,  // ..$J~$@e
                /* 0640 */  0x3D, 0x12, 0x50, 0x10, 0x03, 0xFA, 0xD0, 0x01,  // =.P.....
                /* 0648 */  0xF8, 0xB8, 0x4E, 0x00, 0x8B, 0xCB, 0xA1, 0x27,  // ..N....'
                /* 0650 */  0xCB, 0x43, 0x4F, 0x96, 0x82, 0x78, 0xB2, 0x0E,  // .CO..x..
                /* 0658 */  0x34, 0x59, 0xF4, 0x05, 0xC2, 0x77, 0x29, 0x4F,  // 4Y...w)O
                /* 0660 */  0x16, 0x73, 0xA2, 0x78, 0xE7, 0xC5, 0xDD, 0x8A,  // .s.x....
                /* 0668 */  0x00, 0x2F, 0xFF, 0xFF, 0x1B, 0x05, 0xF0, 0x91,  // ./......
                /* 0670 */  0xE8, 0x54, 0xEA, 0xD5, 0x10, 0x64, 0x52, 0x6E,  // .T...dRn
                /* 0678 */  0x45, 0xE8, 0x40, 0x37, 0x0A, 0x32, 0xA4, 0x77,  // E.@7.2.w
                /* 0680 */  0xB6, 0x43, 0x62, 0x51, 0x6E, 0x14, 0xA8, 0x10,  // .CbQn...
                /* 0688 */  0x14, 0x8A, 0x71, 0x2A, 0x40, 0x09, 0xBF, 0x15,  // ..q*@...
                /* 0690 */  0x51, 0x10, 0xDF, 0x8A, 0x00, 0x27, 0x69, 0x06,  // Q....'i.
                /* 0698 */  0x85, 0xD6, 0x73, 0x2B, 0x82, 0x72, 0x73, 0xC0,  // ..s+.rs.
                /* 06A0 */  0x5E, 0x8B, 0x80, 0xF1, 0xFF, 0xFF, 0x5A, 0x04,  // ^.....Z.
                /* 06A8 */  0x70, 0x4C, 0xA2, 0x45, 0xAD, 0xA7, 0x0E, 0x90,  // pL.E....
                /* 06B0 */  0x05, 0x58, 0x05, 0x3D, 0xCB, 0x58, 0x32, 0x0C,  // .X.=.X2.
                /* 06B8 */  0x4A, 0x02, 0x84, 0xEE, 0xB3, 0x1E, 0x02, 0x1F,  // J.......
                /* 06C0 */  0xA1, 0xE7, 0xDC, 0x3C, 0xBE, 0x2E, 0x10, 0xA7,  // ...<....
                /* 06C8 */  0xE3, 0x93, 0xC6, 0x01, 0x30, 0x69, 0x63, 0x43,  // ....0icC
                /* 06D0 */  0x9D, 0x38, 0x3C, 0xC8, 0x77, 0x09, 0xDF, 0x38,  // .8<.w..8
                /* 06D8 */  0x7C, 0xB1, 0x31, 0x4C, 0x02, 0x8F, 0x8D, 0xA1,  // |.1L....
                /* 06E0 */  0xFD, 0xFF, 0xA3, 0xB1, 0x13, 0x01, 0x0C, 0xC0,  // ........
                /* 06E8 */  0x43, 0x7C, 0x28, 0xF5, 0x20, 0x3C, 0xB3, 0x83,  // C|(. <..
                /* 06F0 */  0xC0, 0x9C, 0x4C, 0x7C, 0xF9, 0x6B, 0xF5, 0x48,  // ..L|.k.H
                /* 06F8 */  0x80, 0x70, 0x7F, 0x23, 0xD0, 0xA1, 0xFA, 0x45,  // .p.#...E
                /* 0700 */  0xE0, 0x09, 0x20, 0x46, 0x10, 0x5B, 0x13, 0x60,  // .. F.[.`
                /* 0708 */  0x6B, 0xF2, 0x02, 0xF2, 0x69, 0x20, 0xCA, 0x29,  // k...i .)
                /* 0710 */  0x05, 0x7A, 0xE9, 0x35, 0x4A, 0x73, 0x02, 0x8C,  // .z.5Js..
                /* 0718 */  0x41, 0x21, 0x40, 0x62, 0xBD, 0x07, 0xD4, 0x06,  // A!@b....
                /* 0720 */  0x26, 0xA8, 0x60, 0x61, 0x82, 0x18, 0xAA, 0xFD,  // &.`a....
                /* 0728 */  0x41, 0x10, 0xF9, 0xCF, 0x00, 0x5D, 0x33, 0x7C,  // A....]3|
                /* 0730 */  0x76, 0xF7, 0x68, 0xF8, 0x41, 0x88, 0x8F, 0xE6,  // v.h.A...
                /* 0738 */  0x54, 0x1F, 0x3E, 0x18, 0xC8, 0x43, 0x8A, 0x4F,  // T.>..C.O
                /* 0740 */  0xAF, 0x87, 0xEB, 0x8B, 0xAA, 0x09, 0x1E, 0x1E,  // ........
                /* 0748 */  0x7C, 0x86, 0x80, 0xE3, 0x1A, 0x50, 0x13, 0x3F,  // |....P.?
                /* 0750 */  0x44, 0xF6, 0x93, 0x31, 0x1C, 0x1F, 0xA2, 0x0F,  // D..1....
                /* 0758 */  0x15, 0x9E, 0xC0, 0xDB, 0x80, 0x01, 0xD9, 0x21,  // .......!
                /* 0760 */  0x02, 0xFE, 0x78, 0x0C, 0xFD, 0x14, 0xE1, 0x1B,  // ..x.....
                /* 0768 */  0xA0, 0xC7, 0xA6, 0x41, 0xA0, 0x4E, 0x1A, 0x7C,  // ...A.N.|
                /* 0770 */  0x98, 0xA7, 0xF5, 0x86, 0xEF, 0xEB, 0xB9, 0x09,  // ........
                /* 0778 */  0x2C, 0xF6, 0xEC, 0x40, 0xC7, 0x03, 0x7E, 0xC5,  // ,..@..~.
                /* 0780 */  0x47, 0x03, 0x5D, 0x5E, 0x8C, 0xED, 0x33, 0x8B,  // G.]^..3.
                /* 0788 */  0x23, 0x1C, 0x44, 0xA0, 0x80, 0x9E, 0x8D, 0xCF,  // #.D.....
                /* 0790 */  0x10, 0xBE, 0x17, 0xB0, 0xA3, 0x05, 0x17, 0x75,  // .......u
                /* 0798 */  0xB4, 0x40, 0x1D, 0x1A, 0x7C, 0x5E, 0x60, 0x88,  // .@..|^`.
                /* 07A0 */  0x8F, 0x33, 0xFE, 0xFF, 0x43, 0x3C, 0x59, 0xB0,  // .3..C<Y.
                /* 07A8 */  0x93, 0x14, 0x38, 0x47, 0xE2, 0x43, 0x08, 0x7C,  // ..8G.C.|
                /* 07B0 */  0x90, 0x97, 0x15, 0x43, 0x04, 0x09, 0xFE, 0x4C,  // ...C...L
                /* 07B8 */  0x70, 0x3A, 0x6C, 0x68, 0x18, 0x82, 0x22, 0x47,  // p:lh.."G
                /* 07C0 */  0x80, 0x3A, 0x76, 0x78, 0x04, 0xFC, 0x9C, 0xF0,  // .:vx....
                /* 07C8 */  0x40, 0xC2, 0x46, 0xF0, 0x4E, 0xE2, 0x49, 0x1C,  // @.F.N.I.
                /* 07D0 */  0x90, 0xCF, 0x16, 0x46, 0x88, 0x5D, 0xEE, 0x11,  // ...F.]..
                /* 07D8 */  0x83, 0x68, 0x1E, 0x9F, 0xCE, 0x30, 0x98, 0x31,  // .h...0.1
                /* 07E0 */  0xE1, 0x4E, 0x0C, 0x1E, 0x02, 0x1F, 0xC0, 0xB3,  // .N......
                /* 07E8 */  0xC5, 0xF9, 0xF9, 0x84, 0x72, 0x56, 0xB8, 0x11,  // ....rV..
                /* 07F0 */  0xF3, 0x29, 0xE1, 0x06, 0x80, 0x51, 0x68, 0xD3,  // .)...Qh.
                /* 07F8 */  0xA7, 0x46, 0xA3, 0x56, 0x0D, 0xCA, 0xD4, 0x28,  // .F.V...(
                /* 0800 */  0xD3, 0xA0, 0x56, 0x9F, 0x4A, 0x8D, 0x19, 0xB3,  // ..V.J...
                /* 0808 */  0x07, 0xA5, 0xE3, 0x94, 0x4F, 0x00, 0x16, 0xF1,  // ....O...
                /* 0810 */  0x08, 0x10, 0x88, 0xC5, 0x52, 0xC8, 0xC8, 0xA8,  // ....R...
                /* 0818 */  0x0C, 0x22, 0x20, 0xAB, 0x7C, 0x97, 0x08, 0xC8,  // ." .|...
                /* 0820 */  0x9A, 0x41, 0x04, 0xE4, 0xD4, 0x16, 0x80, 0x98,  // .A......
                /* 0828 */  0x86, 0x37, 0x86, 0x80, 0x9C, 0xCC, 0x03, 0x10,  // .7......
                /* 0830 */  0xCB, 0x09, 0x22, 0x20, 0x4B, 0x35, 0x01, 0xC4,  // .." K5..
                /* 0838 */  0x74, 0x83, 0x08, 0xC8, 0x5A, 0x55, 0x0C, 0x82,  // t...ZU..
                /* 0840 */  0x82, 0x08, 0xC8, 0x3A, 0x5E, 0x0E, 0x02, 0xB2,  // ...:^...
                /* 0848 */  0x2A, 0x10, 0x01, 0x39, 0x9F, 0x0D, 0x20, 0x16,  // *..9.. .
                /* 0850 */  0x16, 0x44, 0x40, 0x16, 0xA9, 0x03, 0x88, 0x29,  // .D@....)
                /* 0858 */  0x06, 0x11, 0x90, 0xB5, 0xF9, 0x00, 0x62, 0x4A,  // ......bJ
                /* 0860 */  0x41, 0x34, 0x20, 0xF2, 0x28, 0x10, 0x90, 0x25,  // A4 .(..%
                /* 0868 */  0x50, 0x08, 0xC8, 0x71, 0x41, 0x04, 0xE4, 0xF0,  // P..qA...
                /* 0870 */  0x40, 0x34, 0xC7, 0xFB, 0x41, 0x40, 0xFE, 0xFF,  // @4..A@..
                /* 0878 */  0x87, 0x02, 0x11, 0x90, 0x03, 0x3C, 0x1B, 0x04,  // .....<..
                /* 0880 */  0xE4, 0x38, 0x20, 0x02, 0xB2, 0x24, 0x2D, 0x40,  // .8 ..$-@
                /* 0888 */  0x4C, 0x1F, 0x88, 0x80, 0x1C, 0x12, 0x88, 0x26,  // L......&
                /* 0890 */  0x06, 0xA2, 0x89, 0xBD, 0x00, 0xB1, 0xFC, 0x20,  // ....... 
                /* 0898 */  0x02, 0xB2, 0x7E, 0x33, 0x40, 0x2C, 0x0D, 0x88,  // ..~3@,..
                /* 08A0 */  0x80, 0x2C, 0xC1, 0x0D, 0x10, 0xD3, 0x03, 0x22,  // .,....."
                /* 08A8 */  0x20, 0x67, 0x07, 0xA2, 0x4A, 0xDE, 0x05, 0x02,  //  g..J...
                /* 08B0 */  0x26, 0x76, 0x80, 0x58, 0x18, 0x10, 0x01, 0x59,  // &v.X...Y
                /* 08B8 */  0xCA, 0xBB, 0x4B, 0x40, 0xD6, 0x05, 0x22, 0x20,  // ..K@.." 
                /* 08C0 */  0x6B, 0xF0, 0x03, 0xC4, 0x14, 0x81, 0x68, 0x70,  // k.....hp
                /* 08C8 */  0xE4, 0x01, 0xA2, 0xC1, 0x11, 0x10, 0x01, 0x39,  // .......9
                /* 08D0 */  0x86, 0x21, 0x20, 0x0A, 0xA2, 0x01, 0x10, 0x47,  // .! ....G
                /* 08D8 */  0x03, 0xA0, 0x20, 0x02, 0x72, 0x2E, 0x20, 0x1A,  // .. .r. .
                /* 08E0 */  0x12, 0x88, 0x06, 0x7A, 0xFE, 0x08, 0xC8, 0xF1,  // ...z....
                /* 08E8 */  0x40, 0x04, 0x64, 0x19, 0x9E, 0x40, 0x58, 0x08,  // @.d..@X.
                /* 08F0 */  0x10, 0x2A, 0xE3, 0xE5, 0x21, 0x10, 0xFF, 0xFF,  // .*..!...
                /* 08F8 */  0x01                                             // .
            })
            Method (IANE, 1, Serialized)
            {
                IANQ (Arg0)
                Notify (ATKD, 0xFF) // Hardware-Specific
            }

            Method (INIT, 1, NotSerialized)
            {
                Store (One, ATKP) /* \_SB_.ATKP */
                Return (One)
            }

            Method (BSTS, 0, NotSerialized)
            {
                Store (Zero, Local0)
                If (^^PCI0.SBRG.EC0.ACAP ())
                {
                    Store (Zero, Local0)
                }
                Else
                {
                    Store (^^PCI0.SBRG.EC0.ST8E (0x05, Zero), Local0)
                    If (LEqual (And (Local0, 0xFF), 0x34))
                    {
                        Store (0x05, Local0)
                    }
                    Else
                    {
                        Store (Zero, Local0)
                    }
                }

                Return (Local0)
            }

            Method (SFUN, 0, NotSerialized)
            {
                Store (0x21, Local0)
                Return (Local0)
            }

            Method (WDOG, 1, NotSerialized)
            {
                Return (Zero)
            }

            Method (KBNI, 0, NotSerialized)
            {
                Return (One)
            }

            Name (CA1M, Zero)
            Method (RMEM, 1, NotSerialized)
            {
                OperationRegion (VMEM, SystemMemory, Arg0, 0x04)
                Field (VMEM, ByteAcc, NoLock, Preserve)
                {
                    MEMI,   32
                }

                Store (MEMI, Local0)
                Return (Local0)
            }

            Method (WMEM, 2, NotSerialized)
            {
                OperationRegion (VMEM, SystemMemory, Arg0, 0x04)
                Field (VMEM, ByteAcc, NoLock, Preserve)
                {
                    MEMI,   32
                }

                Store (Arg1, MEMI) /* \_SB_.ATKD.WMEM.MEMI */
            }

            Name (MEMD, Package (0x81)
            {
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF
            })
            Method (SCDG, 2, NotSerialized)
            {
                Store (Zero, Index (MEMD, Zero))
                Store (CAL1, CA1M) /* \_SB_.ATKD.CA1M */
                If (LEqual (ToInteger (CAL1), Zero))
                {
                    Return (MEMD) /* \_SB_.ATKD.MEMD */
                }

                If (LNotEqual (Arg0, One))
                {
                    Store (Zero, Index (MEMD, Zero))
                    Return (MEMD) /* \_SB_.ATKD.MEMD */
                }

                Store (Zero, Local2)
                Store (One, Local3)
                Store (CA1M, Local1)
                Store (0x0200, Index (MEMD, Zero))
                While (LLess (Local3, 0x81))
                {
                    Store (RMEM (Add (Local1, Local2)), Index (MEMD, Local3))
                    Add (Local2, 0x04, Local2)
                    Add (Local3, One, Local3)
                }

                Return (MEMD) /* \_SB_.ATKD.MEMD */
            }

            Method (SPEC, 1, NotSerialized)
            {
                If (LEqual (Arg0, Zero))
                {
                    Return (0x00090000)
                }
                ElseIf (LEqual (Arg0, One))
                {
                    Return (One)
                }

                Return (0xFFFFFFFE)
            }

            Method (OSVR, 1, NotSerialized)
            {
                If (LEqual (OSFG, Zero))
                {
                    Store (Arg0, OSFG) /* \OSFG */
                }
            }

            Method (VERS, 2, NotSerialized)
            {
                If (LEqual (Arg0, Zero))
                {
                    Return (0x00090000)
                }

                Return (0xFFFFFFFE)
            }

            Method (GLCD, 0, NotSerialized)
            {
                Return (LCDV) /* \_SB_.LCDV */
            }

            Method (ANVI, 1, Serialized)
            {
                Store (0x04, SMIF) /* \_SB_.SMIF */
                Return (ASMI (Arg0))
            }
        }
    }

    Scope (_SB)
    {
        Device (ASHS)
        {
            Name (_HID, "ATK4002")  // _HID: Hardware ID
            Method (HSWC, 1, Serialized)
            {
                If (LLess (Arg0, 0x02))
                {
                    Return (SWBL (Arg0))
                }

                If (LEqual (Arg0, 0x02))
                {
                    If (LEqual (GWBL (), Zero))
                    {
                        Return (0x04)
                    }
                    Else
                    {
                        Return (0x05)
                    }
                }

                If (LEqual (Arg0, 0x03))
                {
                    Return (0xFF)
                }

                If (LEqual (Arg0, 0x04))
                {
                    Return (SWBL (Zero))
                }

                If (LEqual (Arg0, 0x05))
                {
                    Return (SWBL (One))
                }

                If (LEqual (Arg0, 0x80))
                {
                    Return (One)
                }
            }

            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LGreaterEqual (MSOS (), OSW8))
                {
                    Return (0x0F)
                }
                Else
                {
                    Return (Zero)
                }
            }
        }
    }

    Scope (\)
    {
        Method (OHWS, 0, Serialized)
        {
            Return (Zero)
        }

        Method (GWBL, 0, Serialized)
        {
            Return (\_SB.GGOV (One, One))
        }

        Method (SWBL, 1, Serialized)
        {
            If (LEqual (Arg0, Zero))
            {
                \_SB.SGOV (One, One, Zero)
            }
            ElseIf (LEqual (Arg0, One))
            {
                \_SB.SGOV (One, One, One)
            }

            Return (One)
        }
    }

    Scope (_SB)
    {
        Device (ASUP)
        {
            Name (_HID, "ASUS2018")  // _HID: Hardware ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (0x0F)
            }
        }

        Device (ARLS)
        {
            Name (_HID, "ASUS9001")  // _HID: Hardware ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (ARSE)
                {
                    Return (0x0F)
                }
                Else
                {
                    Return (Zero)
                }
            }
        }
    }

    Scope (\)
    {
        OperationRegion (DBIO, SystemIO, 0x03F8, 0x08)
        Field (DBIO, ByteAcc, NoLock, Preserve)
        {
            SOUT,   8, 
            Offset (0x05), 
            SSAT,   8
        }

        Method (GETC, 2, NotSerialized)
        {
            CreateByteField (Arg0, Arg1, TCHR)
            Return (TCHR) /* \GETC.TCHR */
        }

        Method (THRE, 0, NotSerialized)
        {
            And (SSAT, 0x20, Local0)
            While (LEqual (Local0, Zero))
            {
                And (SSAT, 0x20, Local0)
            }
        }

        Method (OUTX, 1, NotSerialized)
        {
            THRE ()
            Store (Arg0, SOUT) /* \SOUT */
        }

        Method (OUTC, 1, NotSerialized)
        {
            If (LEqual (Arg0, 0x0A))
            {
                OUTX (0x0D)
            }

            OUTX (Arg0)
        }

        Method (DBGN, 1, NotSerialized)
        {
            And (Arg0, 0x0F, Local0)
            If (LLess (Local0, 0x0A))
            {
                Add (Local0, 0x30, Local0)
            }
            Else
            {
                Add (Local0, 0x37, Local0)
            }

            OUTC (Local0)
        }

        Method (ADGB, 1, NotSerialized)
        {
            ShiftRight (Arg0, 0x04, Local0)
            DBGN (Local0)
            DBGN (Arg0)
        }

        Method (ADGW, 1, NotSerialized)
        {
            ShiftRight (Arg0, 0x08, Local0)
            ADGB (Local0)
            ADGB (Arg0)
        }

        Method (ADGD, 1, NotSerialized)
        {
            ShiftRight (Arg0, 0x10, Local0)
            ADGW (Local0)
            ADGW (Arg0)
        }

        Method (ADGQ, 1, NotSerialized)
        {
            ShiftRight (Arg0, 0x30, Local0)
            ADGW (Local0)
            ShiftRight (Arg0, 0x20, Local0)
            ADGW (Local0)
            ShiftRight (Arg0, 0x10, Local0)
            ADGW (Local0)
            ADGW (Arg0)
        }

        Name (DBGZ, Buffer (0x50){})
        Method (AMLI, 1, NotSerialized)
        {
            If (LEqual (ObjectType (Arg0), One))
            {
                If (LGreater (Arg0, 0xFFFFFFFF))
                {
                    ADGQ (Arg0)
                }
                ElseIf (LGreater (Arg0, 0xFFFF))
                {
                    ADGD (Arg0)
                }
                ElseIf (LGreater (Arg0, 0xFF))
                {
                    ADGW (Arg0)
                }
                Else
                {
                    ADGB (Arg0)
                }

                Return (Zero)
            }

            If (LEqual (ObjectType (Arg0), 0x02))
            {
                Store (Arg0, DBGZ) /* \DBGZ */
                Store (Zero, Local1)
                While (One)
                {
                    Store (GETC (DBGZ, Local1), Local0)
                    If (LEqual (Local0, Zero))
                    {
                        Return (Zero)
                    }

                    OUTC (Local0)
                    Increment (Local1)
                }

                Return (Zero)
            }

            If (LEqual (ObjectType (Arg0), 0x03))
            {
                Store (Zero, Local0)
                While (LLess (Local0, SizeOf (Arg0)))
                {
                    Store (GETC (Arg0, Local0), Local1)
                    If (LGreater (Local0, Zero))
                    {
                        OUTC (0x20)
                    }

                    ADGB (Local1)
                    Increment (Local0)
                }

                Return (Zero)
            }

            If (LEqual (ObjectType (Arg0), 0x04))
            {
                Store (Zero, Local0)
                While (LLess (Local0, SizeOf (Arg0)))
                {
                    OUTC (0x20)
                    OUTC (0x20)
                    AMLI (DerefOf (Index (Arg0, Local0)))
                    OUTC (0x0A)
                    Increment (Local0)
                }

                Return (Zero)
            }

            Return (One)
        }
    }

    OperationRegion (ASDB, SystemIO, 0xB2, 0x02)
    Field (ASDB, ByteAcc, Lock, Preserve)
    {
        ASB2,   8, 
        ASB3,   8
    }

    Method (SBDS, 1, NotSerialized)
    {
        Store (ASB3, Local0)
        Store (Add (0xD0, Arg0), ASB3) /* \ASB3 */
        Store (0xE0, ASB2) /* \ASB2 */
        Store (Local0, ASB3) /* \ASB3 */
    }

    Method (SBDW, 1, NotSerialized)
    {
        Store (ASB3, Local0)
        Store (Add (0xF0, Arg0), ASB3) /* \ASB3 */
        Store (0xE0, ASB2) /* \ASB2 */
        Store (Local0, ASB3) /* \ASB3 */
    }

    Scope (_SB.PCI0.SBRG)
    {
        Device (EC0)
        {
            Name (_HID, EisaId ("PNP0C09") /* Embedded Controller Device */)  // _HID: Hardware ID
            Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
            {
                IO (Decode16,
                    0x0062,             // Range Minimum
                    0x0062,             // Range Maximum
                    0x00,               // Alignment
                    0x01,               // Length
                    )
                IO (Decode16,
                    0x0066,             // Range Minimum
                    0x0066,             // Range Maximum
                    0x00,               // Alignment
                    0x01,               // Length
                    )
            })
            Method (_GPE, 0, NotSerialized)  // _GPE: General Purpose Events
            {
                Store (0x03, Local0)
                Return (Local0)
            }

            Mutex (MUEC, 0x00)
            Mutex (MU4T, 0x00)
            Mutex (CMUT, 0x00)
            Name (ECFL, Ones)
            Method (ECAV, 0, NotSerialized)
            {
                If (LEqual (ECFL, Ones))
                {
                    If (LGreaterEqual (_REV, 0x02))
                    {
                        Return (One)
                    }
                    Else
                    {
                        Return (Zero)
                    }

                    Return (Zero)
                }

                Return (ECFL) /* \_SB_.PCI0.SBRG.EC0_.ECFL */
            }

            OperationRegion (BRIH, SystemIO, 0x0381, One)
            Field (BRIH, ByteAcc, Lock, Preserve)
            {
                BRAH,   8
            }

            OperationRegion (BRAM, SystemIO, 0x0382, 0x02)
            Field (BRAM, ByteAcc, Lock, Preserve)
            {
                BRAI,   8, 
                BRAD,   8
            }

            IndexField (BRAI, BRAD, ByteAcc, NoLock, Preserve)
            {
                Offset (0x04), 
                CCI0,   8, 
                CCI1,   8, 
                CCI2,   8, 
                CCI3,   8, 
                CTL0,   8, 
                CTL1,   8, 
                CTL2,   8, 
                CTL3,   8, 
                CTL4,   8, 
                CTL5,   8, 
                CTL6,   8, 
                CTL7,   8, 
                MGI0,   8, 
                MGI1,   8, 
                MGI2,   8, 
                MGI3,   8, 
                MGI4,   8, 
                MGI5,   8, 
                MGI6,   8, 
                MGI7,   8, 
                MGI8,   8, 
                MGI9,   8, 
                MGIA,   8, 
                MGIB,   8, 
                MGIC,   8, 
                MGID,   8, 
                MGIE,   8, 
                MGIF,   8, 
                MGO0,   8, 
                MGO1,   8, 
                MGO2,   8, 
                MGO3,   8, 
                MGO4,   8, 
                MGO5,   8, 
                MGO6,   8, 
                MGO7,   8, 
                MGO8,   8, 
                MGO9,   8, 
                MGOA,   8, 
                MGOB,   8, 
                MGOC,   8, 
                MGOD,   8, 
                MGOE,   8, 
                MGOF,   8, 
                Offset (0x40), 
                BRD1,   8, 
                BRD2,   8, 
                BRD3,   8, 
                BRD4,   8, 
                Offset (0x6E), 
                BFLB,   8, 
                BCLB,   8, 
                Offset (0x90), 
                EPWS,   8, 
                EB0S,   8, 
                EB1S,   8, 
                EB0T,   8, 
                EB1T,   8, 
                Offset (0x98), 
                ECPU,   8, 
                ECRT,   8, 
                EPSV,   8, 
                PDST,   8, 
                THL0,   8, 
                THH0,   8, 
                THL1,   8, 
                THH1,   8, 
                B0VL,   16, 
                B0RC,   16, 
                B0FC,   16, 
                B0MD,   16, 
                B0ST,   16, 
                B0CC,   16, 
                B0DC,   16, 
                B0DV,   16, 
                B1VL,   16, 
                B1RC,   16, 
                B1FC,   16, 
                B1MD,   16, 
                B1ST,   16, 
                B1CC,   16, 
                B1DC,   16, 
                B1DV,   16
            }

            Method (_INI, 0, NotSerialized)  // _INI: Initialize
            {
                KINI ()
                DINI ()
            }

            Method (_REG, 2, NotSerialized)  // _REG: Region Availability
            {
                If (LEqual (Arg0, 0x03))
                {
                    Store (Arg1, ECFL) /* \_SB_.PCI0.SBRG.EC0_.ECFL */
                }
            }
        }
    }

    Scope (_SB.PCI0.SBRG.EC0)
    {
        Device (AC0)
        {
            Name (_HID, "ACPI0003" /* Power Source Device */)  // _HID: Hardware ID
            Name (XX00, Buffer (0x03){})
            Name (ACDC, 0xFF)
            Method (_PSR, 0, NotSerialized)  // _PSR: Power Source
            {
                Store (ACPS (), ACPF) /* \_SB_.ACPF */
                If (LOr (LNotEqual (ACPF, ACDC), LEqual (ACDC, 0xFF)))
                {
                    CreateWordField (XX00, Zero, SSZE)
                    CreateByteField (XX00, 0x02, ACST)
                    Store (0x03, SSZE) /* \_SB_.PCI0.SBRG.EC0_.AC0_._PSR.SSZE */
                    If (ACPF)
                    {
                        AFN4 (One)
                        Store (Zero, ACST) /* \_SB_.PCI0.SBRG.EC0_.AC0_._PSR.ACST */
                    }
                    Else
                    {
                        AFN4 (0x02)
                        Store (One, ACST) /* \_SB_.PCI0.SBRG.EC0_.AC0_._PSR.ACST */
                    }

                    ALIB (One, XX00)
                    Store (ACPF, ACDC) /* \_SB_.PCI0.SBRG.EC0_.AC0_.ACDC */
                }

                Return (ACPF) /* \_SB_.ACPF */
            }

            Name (_PCL, Package (0x01)  // _PCL: Power Consumer List
            {
                PCI0
            })
        }

        Device (LID)
        {
            Name (_HID, EisaId ("PNP0C0D") /* Lid Device */)  // _HID: Hardware ID
            Method (_LID, 0, Serialized)  // _LID: Lid Status
            {
                Store (One, Local0)
                Store (RPIN (0x11), Local0)
                If (LEqual (Local0, Ones))
                {
                    Store (One, Local0)
                }

                Name (DPTI, Buffer (0x07){})
                CreateWordField (DPTI, Zero, SSZE)
                CreateByteField (DPTI, 0x02, MSID)
                CreateDWordField (DPTI, 0x03, DECI)
                If (LEqual (Local0, Zero))
                {
                    Store (0x52, Local1)
                    ^^^^^ATKD.IANE (0xE9)
                }
                Else
                {
                    Switch (CFML)
                    {
                        Case (One)
                        {
                            If (LEqual (WRFG, One))
                            {
                                Store (0x5A, Local1)
                            }
                            Else
                            {
                                Store (0x5E, Local1)
                            }
                        }
                        Case (0x02)
                        {
                            Store (0x5A, Local1)
                        }
                        Case (0x04)
                        {
                            Store (0x60, Local1)
                        }
                        Default
                        {
                            Store (0x5A, Local1)
                        }

                    }
                }

                Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_.LID_._LID.SSZE */
                Store (0x03, MSID) /* \_SB_.PCI0.SBRG.EC0_.LID_._LID.MSID */
                Store (Local1, DECI) /* \_SB_.PCI0.SBRG.EC0_.LID_._LID.DECI */
                ALIB (0x0C, DPTI)
                Return (Local0)
            }
        }
    }

    Scope (_SB.PCI0.SBRG.EC0)
    {
        OperationRegion (ECAD, SystemMemory, EGBF, 0x10)
        Field (ECAD, DWordAcc, NoLock, Preserve)
        {
            MFUN,   16, 
            SFUN,   16, 
            LEN,    16, 
            STAS,   8, 
            EROR,   8, 
            CMD,    8, 
            EDA1,   8, 
            EDA2,   8, 
            EDA3,   8, 
            EDA4,   8, 
            EDA5,   8, 
            Offset (0x10)
        }

        Name (CUNT, Zero)
        Method (ECAC, 0, NotSerialized)
        {
            Store (0x30, MFUN) /* \_SB_.PCI0.SBRG.EC0_.MFUN */
            Store (One, SFUN) /* \_SB_.PCI0.SBRG.EC0_.SFUN */
            Store (0x10, LEN) /* \_SB_.PCI0.SBRG.EC0_.LEN_ */
            Store (0xFF, EROR) /* \_SB_.PCI0.SBRG.EC0_.EROR */
            Store (One, CUNT) /* \_SB_.PCI0.SBRG.EC0_.CUNT */
            While (LLess (CUNT, 0x06))
            {
                ISMI (0x9C)
                If (LNotEqual (EROR, Zero))
                {
                    Add (CUNT, One, CUNT) /* \_SB_.PCI0.SBRG.EC0_.CUNT */
                }
                Else
                {
                    Break
                }
            }
        }

        Method (ST83, 1, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0x83, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                ECAC ()
                Store (EDA1, Local0)
                Release (MU4T)
                Return (Local0)
            }

            Return (Ones)
        }

        Method (ST84, 2, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0x84, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                Store (Arg1, EDA3) /* \_SB_.PCI0.SBRG.EC0_.EDA3 */
                ECAC ()
                Release (MU4T)
                Return (Zero)
            }

            Return (Ones)
        }

        Method (ST87, 2, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0x87, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                Store (Arg1, EDA3) /* \_SB_.PCI0.SBRG.EC0_.EDA3 */
                ECAC ()
                Store (EDA1, Local0)
                Release (MU4T)
                If (LEqual (Arg0, Zero))
                {
                    Return (Local0)
                }

                Return (Zero)
            }

            Return (Ones)
        }

        Method (ST8E, 2, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0x8E, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                Store (Arg1, EDA3) /* \_SB_.PCI0.SBRG.EC0_.EDA3 */
                ECAC ()
                Store (EDA1, Local0)
                Release (MU4T)
                Return (Local0)
            }

            Return (Ones)
        }

        Method (ST95, 2, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0x95, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                Store (Arg1, EDA3) /* \_SB_.PCI0.SBRG.EC0_.EDA3 */
                ECAC ()
                Store (EDA1, Local0)
                Release (MU4T)
                If (LEqual (Arg0, Zero))
                {
                    Return (Local0)
                }

                Return (Zero)
            }

            Return (Ones)
        }

        Method (ST98, 1, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0x98, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                ECAC ()
                Release (MU4T)
                Return (Zero)
            }

            Return (Ones)
        }

        Method (ST9E, 3, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0x9E, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                Store (Arg1, EDA3) /* \_SB_.PCI0.SBRG.EC0_.EDA3 */
                Store (Arg2, EDA4) /* \_SB_.PCI0.SBRG.EC0_.EDA4 */
                ECAC ()
                Release (MU4T)
                Return (Zero)
            }

            Return (Ones)
        }

        Method (ST9F, 3, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0x9F, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                Store (Arg1, EDA3) /* \_SB_.PCI0.SBRG.EC0_.EDA3 */
                Store (Arg2, EDA4) /* \_SB_.PCI0.SBRG.EC0_.EDA4 */
                ECAC ()
                Release (MU4T)
                Return (Zero)
            }

            Return (Ones)
        }

        Method (STA8, 1, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0xA8, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                ECAC ()
                Store (EDA1, Local0)
                Release (MU4T)
                Return (Local0)
            }

            Return (Ones)
        }

        Method (STA9, 2, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0xA9, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                Store (Arg1, EDA3) /* \_SB_.PCI0.SBRG.EC0_.EDA3 */
                ECAC ()
                Release (MU4T)
                Return (Zero)
            }

            Return (Ones)
        }

        Method (STB0, 1, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0xB0, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                ECAC ()
                Release (MU4T)
                Return (Zero)
            }

            Return (Ones)
        }

        Method (STB1, 1, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0xB1, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                ECAC ()
                Release (MU4T)
                Return (Zero)
            }

            Return (Ones)
        }

        Method (STB2, 1, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0xB2, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                ECAC ()
                Release (MU4T)
                Return (Zero)
            }

            Return (Ones)
        }

        Method (SPIN, 2, Serialized)
        {
            If (Arg1)
            {
                ST87 (0x20, Arg0)
            }
            Else
            {
                ST87 (0x40, Arg0)
            }

            Return (One)
        }

        Method (RPIN, 1, Serialized)
        {
            Store (ST87 (Zero, Arg0), Local1)
            Return (Local1)
        }

        Method (RRAM, 2, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (Arg1, Local0)
                And (Local0, 0xFF, Local1)
                ShiftRight (Local0, 0x08, Local0)
                And (Local0, 0xFF, Local0)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (Arg0, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Local0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                Store (Local1, EDA3) /* \_SB_.PCI0.SBRG.EC0_.EDA3 */
                ECAC ()
                Store (EDA1, Local0)
                Release (MU4T)
                Return (Local0)
            }

            Return (Ones)
        }

        Method (WRAM, 3, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (Arg1, Local0)
                And (Local0, 0xFF, Local1)
                ShiftRight (Local0, 0x08, Local0)
                And (Local0, 0xFF, Local0)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (Arg0, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Local0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                Store (Local1, EDA3) /* \_SB_.PCI0.SBRG.EC0_.EDA3 */
                Store (Arg2, EDA4) /* \_SB_.PCI0.SBRG.EC0_.EDA4 */
                ECAC ()
                Release (MU4T)
                Return (One)
            }

            Return (Ones)
        }

        Method (RP2E, 1, NotSerialized)
        {
            Store (Acquire (CMUT, 0xFFFF), Local1)
            If (LEqual (Local1, Zero))
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (Arg0, BRAI) /* \_SB_.PCI0.SBRG.EC0_.BRAI */
                Store (BRAD, Local0)
                Release (CMUT)
            }

            Return (Local0)
        }

        Method (WP2E, 2, NotSerialized)
        {
            Store (Acquire (CMUT, 0xFFFF), Local1)
            If (LEqual (Local1, Zero))
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (Arg0, BRAI) /* \_SB_.PCI0.SBRG.EC0_.BRAI */
                Store (Arg1, BRAD) /* \_SB_.PCI0.SBRG.EC0_.BRAD */
                Release (CMUT)
            }
        }

        Method (WEBC, 3, NotSerialized)
        {
            Store (Acquire (CMUT, 0xFFFF), Local0)
            If (LEqual (Local0, Zero))
            {
                Store (0xC8, Local1)
                While (LAnd (BCLB, Local1))
                {
                    Stall (0x64)
                    Decrement (Local1)
                }

                If (LNot (Local1))
                {
                    Return (Zero)
                }

                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (Zero, Local0)
                While (LLess (Local0, Arg1))
                {
                    Store (Add (0x40, Local0), BRAI) /* \_SB_.PCI0.SBRG.EC0_.BRAI */
                    Store (DerefOf (Index (Arg2, Local0)), BRAD) /* \_SB_.PCI0.SBRG.EC0_.BRAD */
                    Increment (Local0)
                }

                Or (BCLB, 0x80, BCLB) /* \_SB_.PCI0.SBRG.EC0_.BCLB */
                Store (Arg0, BFLB) /* \_SB_.PCI0.SBRG.EC0_.BFLB */
                Release (CMUT)
                Return (One)
            }
            Else
            {
                Return (Zero)
            }
        }

        Method (REBC, 2, Serialized)
        {
            Name (TMBF, Buffer (Arg1)
            {
                 0x00                                             // .
            })
            Store (Acquire (CMUT, 0xFFFF), Local0)
            If (LEqual (Local0, Zero))
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (0xC8, Local1)
                While (LAnd (LGreater (BCLB, Zero), LGreater (Local1, Zero)))
                {
                    Stall (0x64)
                    Decrement (Local1)
                }

                If (LNot (Local1))
                {
                    Release (CMUT)
                    Or (BCLB, 0x40, BCLB) /* \_SB_.PCI0.SBRG.EC0_.BCLB */
                    Return (TMBF) /* \_SB_.PCI0.SBRG.EC0_.REBC.TMBF */
                }

                Or (BCLB, 0x20, BCLB) /* \_SB_.PCI0.SBRG.EC0_.BCLB */
                Store (Arg0, BFLB) /* \_SB_.PCI0.SBRG.EC0_.BFLB */
                Store (0x01F4, Local1)
                While (LNotEqual (And (BCLB, 0x80), 0x80))
                {
                    Stall (0x64)
                    Decrement (Local1)
                    If (LNot (Local1))
                    {
                        Release (CMUT)
                        Return (TMBF) /* \_SB_.PCI0.SBRG.EC0_.REBC.TMBF */
                    }
                }

                Store (Zero, Local0)
                While (LLess (Local0, Arg1))
                {
                    Store (Add (0x40, Local0), BRAI) /* \_SB_.PCI0.SBRG.EC0_.BRAI */
                    Store (BRAD, Index (TMBF, Local0))
                    Increment (Local0)
                }

                Or (BCLB, 0x40, BCLB) /* \_SB_.PCI0.SBRG.EC0_.BCLB */
                Release (CMUT)
                Return (TMBF) /* \_SB_.PCI0.SBRG.EC0_.REBC.TMBF */
            }
            Else
            {
                Return (TMBF) /* \_SB_.PCI0.SBRG.EC0_.REBC.TMBF */
            }
        }

        Method (STCC, 2, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0xCC, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                Store (Arg1, EDA3) /* \_SB_.PCI0.SBRG.EC0_.EDA3 */
                ECAC ()
                Store (EDA1, Local0)
                Release (MU4T)
                Return (Local0)
            }

            Return (Ones)
        }

        Method (STCD, 3, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0xCD, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                Store (Arg1, EDA3) /* \_SB_.PCI0.SBRG.EC0_.EDA3 */
                Store (Arg2, EDA4) /* \_SB_.PCI0.SBRG.EC0_.EDA4 */
                ECAC ()
                Release (MU4T)
                Return (Zero)
            }

            Return (Ones)
        }

        Method (STE7, 1, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (0xFF, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (0xE7, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                Store (Arg0, EDA2) /* \_SB_.PCI0.SBRG.EC0_.EDA2 */
                ECAC ()
                Release (MU4T)
                Return (Zero)
            }

            Return (Ones)
        }
    }

    Scope (_SB.PCI0.SBRG.EC0)
    {
        Method (GPWS, 0, Serialized)
        {
            Acquire (CMUT, 0xFFFF)
            Store (EPWS, Local0)
            Release (CMUT)
            Return (Local0)
        }

        Method (ACPS, 0, Serialized)
        {
            Return (And (GPWS (), One))
        }

        Method (ACAP, 0, Serialized)
        {
            Return (ACPF) /* \_SB_.ACPF */
        }

        Method (DCPS, 1, Serialized)
        {
            Store (GPWS (), Local0)
            If (Arg0)
            {
                And (Local0, 0x04, Local0)
            }
            Else
            {
                And (Local0, 0x02, Local0)
            }

            If (Local0)
            {
                Store (One, Local0)
            }
            Else
            {
                Store (Zero, Local0)
            }

            Return (Local0)
        }

        Method (BCHG, 1, Serialized)
        {
            Acquire (CMUT, 0xFFFF)
            If (Arg0)
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (EB1S, Local0)
                Release (CMUT)
                And (Local0, 0xFF, Local0)
                If (LNotEqual (Local0, 0xFF))
                {
                    And (Local0, 0x02, Local0)
                }
                Else
                {
                    Store (Zero, Local0)
                }

                Return (Local0)
            }
            Else
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (EB0S, Local0)
                Release (CMUT)
                And (Local0, 0xFF, Local0)
                If (LNotEqual (Local0, 0xFF))
                {
                    And (Local0, 0x02, Local0)
                }
                Else
                {
                    Store (Zero, Local0)
                }

                Return (Local0)
            }
        }

        Method (BCLE, 1, Serialized)
        {
            If (Arg0)
            {
                If (ECAV ())
                {
                    Store (Ones, Local0)
                    Acquire (CMUT, 0xFFFF)
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (EB1S, Local1)
                    Release (CMUT)
                    And (Local1, 0xFFFF, Local1)
                    If (LNotEqual (Local1, 0xFFFF))
                    {
                        And (Local1, 0x16, Local1)
                        If (LEqual (Local1, 0x04))
                        {
                            Store (Zero, Local0)
                        }
                        ElseIf (LEqual (Local1, 0x02))
                        {
                            Store (One, Local0)
                        }
                        ElseIf (LEqual (Local1, 0x10))
                        {
                            Store (One, Local0)
                        }
                    }
                }
                Else
                {
                    Store (Ones, Local0)
                }

                Return (Local0)
            }
            Else
            {
                If (ECAV ())
                {
                    Store (Ones, Local0)
                    Acquire (CMUT, 0xFFFF)
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (EB0S, Local1)
                    Release (CMUT)
                    And (Local1, 0xFFFF, Local1)
                    If (LNotEqual (Local1, 0xFFFF))
                    {
                        And (Local1, 0x16, Local1)
                        If (LEqual (Local1, 0x04))
                        {
                            Store (Zero, Local0)
                        }
                        ElseIf (LEqual (Local1, 0x02))
                        {
                            Store (One, Local0)
                        }
                        ElseIf (LEqual (Local1, 0x10))
                        {
                            Store (One, Local0)
                        }
                    }
                }
                Else
                {
                    Store (Ones, Local0)
                }

                Return (Local0)
            }
        }

        Method (GBTT, 1, Serialized)
        {
            Acquire (CMUT, 0xFFFF)
            If (LEqual (Arg0, Zero))
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (EB0T, Local0)
            }
            Else
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (EB1T, Local0)
            }

            Release (CMUT)
            Return (Local0)
        }

        Method (SRSC, 1, NotSerialized)
        {
            Name (DUBF, Buffer (One)
            {
                 0x00                                             // .
            })
            Store (Arg0, Index (DUBF, Zero))
            Return (WEBC (0x06, One, DUBF))
        }

        Method (GDFC, 1, NotSerialized)
        {
            Name (DUBF, Buffer (0x10)
            {
                /* 0000 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                /* 0008 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00   // ........
            })
            Name (DFC0, Buffer (One)
            {
                 0x00                                             // .
            })
            Store (Arg0, Index (DFC0, Zero))
            WEBC (0x20, One, DFC0)
            Store (REBC (0x21, 0x10), DUBF) /* \_SB_.PCI0.SBRG.EC0_.GDFC.DUBF */
            Return (DUBF) /* \_SB_.PCI0.SBRG.EC0_.GDFC.DUBF */
        }

        Method (SUFC, 5, NotSerialized)
        {
            Name (DUBF, Buffer (0x10)
            {
                /* 0000 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                /* 0008 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00   // ........
            })
            Name (UFC0, Buffer (One)
            {
                 0x00                                             // .
            })
            Store (ShiftRight (Arg0, Zero), Index (DUBF, Zero))
            Store (ShiftRight (Arg0, 0x08), Index (DUBF, One))
            Store (ShiftRight (Arg0, 0x10), Index (DUBF, 0x02))
            Store (ShiftRight (Arg0, 0x18), Index (DUBF, 0x03))
            Store (ShiftRight (Arg1, Zero), Index (DUBF, 0x04))
            Store (ShiftRight (Arg1, 0x08), Index (DUBF, 0x05))
            Store (ShiftRight (Arg1, 0x10), Index (DUBF, 0x06))
            Store (ShiftRight (Arg1, 0x18), Index (DUBF, 0x07))
            Store (ShiftRight (Arg2, Zero), Index (DUBF, 0x08))
            Store (ShiftRight (Arg2, 0x08), Index (DUBF, 0x09))
            Store (ShiftRight (Arg2, 0x10), Index (DUBF, 0x0A))
            Store (ShiftRight (Arg2, 0x18), Index (DUBF, 0x0B))
            Store (ShiftRight (Arg3, Zero), Index (DUBF, 0x0C))
            Store (ShiftRight (Arg3, 0x08), Index (DUBF, 0x0D))
            Store (ShiftRight (Arg3, 0x10), Index (DUBF, 0x0E))
            Store (ShiftRight (Arg3, 0x18), Index (DUBF, 0x0F))
            Store (Arg4, Index (UFC0, Zero))
            WEBC (0x20, One, UFC0)
            Return (WEBC (0x22, 0x10, DUBF))
        }

        Method (GFLB, 1, NotSerialized)
        {
            Name (DUBF, Buffer (0x10)
            {
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00   // ........
            })
            Name (FLB0, Buffer (One)
            {
                 0x00                                             // .
            })
            Store (Arg0, Index (FLB0, Zero))
            WEBC (0x20, One, FLB0)
            Store (REBC (0x24, 0x08), DUBF) /* \_SB_.PCI0.SBRG.EC0_.GFLB.DUBF */
            Return (DUBF) /* \_SB_.PCI0.SBRG.EC0_.GFLB.DUBF */
        }

        Method (RTAH, 1, NotSerialized)
        {
            If (LEqual (Arg0, Zero))
            {
                Store (Acquire (CMUT, 0xFFFF), Local2)
                If (LEqual (Local2, Zero))
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (THH0, Local0)
                    ShiftLeft (Local0, 0x08, Local0)
                    Store (THL0, Local1)
                    Or (Local0, Local1, Local0)
                    Release (CMUT)
                }
            }
            ElseIf (LEqual (Arg0, One))
            {
                Store (Acquire (CMUT, 0xFFFF), Local2)
                If (LEqual (Local2, Zero))
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (THH1, Local0)
                    ShiftLeft (Local0, 0x08, Local0)
                    Store (THL1, Local1)
                    Or (Local0, Local1, Local0)
                    Release (CMUT)
                }
            }
            Else
            {
                Store (0xFFFF, Local0)
            }

            Return (Local0)
        }

        Method (TACH, 1, Serialized)
        {
            If (ECAV ())
            {
                Switch (Arg0)
                {
                    Case (Zero)
                    {
                        Store (RTAH (Zero), Local0)
                        Break
                    }
                    Case (One)
                    {
                        Store (RTAH (One), Local0)
                        Break
                    }
                    Default
                    {
                        Return (Ones)
                    }

                }

                Multiply (Local0, 0x02, Local0)
                If (LNotEqual (Local0, Zero))
                {
                    Divide (0x0041CDB4, Local0, , Local0)
                    Return (Local0)
                }
                Else
                {
                    Return (Ones)
                }
            }
            Else
            {
                Return (Ones)
            }
        }

        Method (DINI, 0, Serialized)
        {
            Name (DPTI, Buffer (0x07){})
            CreateWordField (DPTI, Zero, SSZE)
            CreateByteField (DPTI, 0x02, MSID)
            CreateDWordField (DPTI, 0x03, DECI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_.DINI.SSZE */
            Store (0x05, MSID) /* \_SB_.PCI0.SBRG.EC0_.DINI.MSID */
            Store (PL1V, DECI) /* \_SB_.PCI0.SBRG.EC0_.DINI.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_.DINI.SSZE */
            Store (0x07, MSID) /* \_SB_.PCI0.SBRG.EC0_.DINI.MSID */
            Store (PL2V, DECI) /* \_SB_.PCI0.SBRG.EC0_.DINI.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_.DINI.SSZE */
            Store (0x06, MSID) /* \_SB_.PCI0.SBRG.EC0_.DINI.MSID */
            Store (FPTV, DECI) /* \_SB_.PCI0.SBRG.EC0_.DINI.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_.DINI.SSZE */
            Store (0x03, MSID) /* \_SB_.PCI0.SBRG.EC0_.DINI.MSID */
            Store (0x64, DECI) /* \_SB_.PCI0.SBRG.EC0_.DINI.DECI */
            ALIB (0x0C, DPTI)
        }

        Method (KINI, 0, Serialized)
        {
            STB1 (0x02)
            STB1 (0x04)
            Store (Zero, FNIV) /* \_SB_.FNIV */
        }

        Name (KLDT, Zero)
        Name (PWKB, Buffer (0x04)
        {
             0x00, 0x55, 0xAA, 0xFF                           // .U..
        })
        Method (GLID, 0, Serialized)
        {
            Return (RPIN (0x11))
        }

        Method (GLKB, 1, NotSerialized)
        {
            If (LEqual (Arg0, One))
            {
                And (KBLC, 0x80, Local0)
                If (Local0)
                {
                    Return (One)
                }
                Else
                {
                    Return (Zero)
                }
            }
            ElseIf (LEqual (Arg0, 0x02))
            {
                Return (KBLV) /* \_SB_.KBLV */
            }
            ElseIf (LEqual (Arg0, 0x03))
            {
                Return (0x80)
            }

            Return (Ones)
        }

        Method (SLKB, 1, NotSerialized)
        {
            Store (And (Arg0, 0x7F), KBLV) /* \_SB_.KBLV */
            If (And (Arg0, 0x80))
            {
                Store (DerefOf (Index (PWKB, KBLV)), Local0)
            }
            Else
            {
                Store (Zero, Local0)
            }

            Return (One)
        }

        Method (CSEE, 1, Serialized)
        {
            If (ECAV ())
            {
                Acquire (MU4T, 0xFFFF)
                Store (Arg0, CMD) /* \_SB_.PCI0.SBRG.EC0_.CMD_ */
                Store (Arg0, EDA1) /* \_SB_.PCI0.SBRG.EC0_.EDA1 */
                ECAC ()
                Release (MU4T)
                Return (Zero)
            }

            Return (Ones)
        }

        Method (EC0S, 1, NotSerialized)
        {
            If (LEqual (Arg0, 0x03))
            {
                Name (DPTI, Buffer (0x07){})
                CreateWordField (DPTI, Zero, SSZE)
                CreateByteField (DPTI, 0x02, MSID)
                CreateDWordField (DPTI, 0x03, DECI)
                Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_.EC0S.SSZE */
                Store (0x05, MSID) /* \_SB_.PCI0.SBRG.EC0_.EC0S.MSID */
                Store (0x2EE0, DECI) /* \_SB_.PCI0.SBRG.EC0_.EC0S.DECI */
                ALIB (0x0C, DPTI)
                Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_.EC0S.SSZE */
                Store (0x07, MSID) /* \_SB_.PCI0.SBRG.EC0_.EC0S.MSID */
                Store (0x2EE0, DECI) /* \_SB_.PCI0.SBRG.EC0_.EC0S.DECI */
                ALIB (0x0C, DPTI)
                Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_.EC0S.SSZE */
                Store (0x06, MSID) /* \_SB_.PCI0.SBRG.EC0_.EC0S.MSID */
                Store (0x2EE0, DECI) /* \_SB_.PCI0.SBRG.EC0_.EC0S.DECI */
                ALIB (0x0C, DPTI)
            }

            If (LOr (LEqual (Arg0, 0x03), LEqual (Arg0, 0x04)))
            {
                ST9E (0x1F, 0xFF, Zero)
            }

            If (LEqual (Arg0, 0x05))
            {
                ST9E (0x1F, 0xFF, Zero)
            }
        }

        Method (EC0W, 1, NotSerialized)
        {
            KINI ()
            If (LLessEqual (Arg0, 0x04))
            {
                Store (DCPS (Zero), DCPF) /* \_SB_.DCPF */
                Store (ACPS (), Local0)
                If (LNotEqual (Local0, ACPF))
                {
                    Store (ACPS (), ACPF) /* \_SB_.ACPF */
                }
            }

            If (LOr (LEqual (Arg0, 0x03), LEqual (Arg0, 0x04))){}
            If (And (WOLO, One))
            {
                ST9E (0x3B, 0x02, 0x02)
            }

            If (And (DSYN, One))
            {
                ST9F (0x2B, 0x20, 0x20)
            }

            Store (0xFF, ^AC0.ACDC) /* \_SB_.PCI0.SBRG.EC0_.AC0_.ACDC */
        }

        Name (PL1V, 0x7530)
        Name (PL2V, 0x9C40)
        Name (PL1B, 0x88B8)
        Name (PL2B, 0xD2F0)
        Name (PL1S, 0x7A12)
        Name (PL2S, 0xAAE6)
        Name (PL1O, 0x7530)
        Name (PL2O, 0x9C40)
        Name (FPTV, 0xD6D8)
        Name (FPTB, 0xFDE8)
        Name (FPTS, 0xF424)
        Name (DGD1, 0xD1)
        Name (DGD2, 0xD2)
        Name (DGD3, 0xD3)
        Name (DGD4, 0xD4)
        Name (DGD5, 0xD4)
        Method (DCPL, 0, Serialized)
        {
            Acquire (CMUT, 0xFFFF)
            Store (PDST, Local0)
            Release (CMUT)
            If (LEqual (ACPS (), Zero))
            {
                Switch (CFML)
                {
                    Case (0x02)
                    {
                        Store (0x4E20, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0x61A8, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0x61A8, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }
                    Default
                    {
                        Store (0x4E20, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0x61A8, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0x61A8, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }

                }
            }
            ElseIf (LAnd (And (Local0, 0x02), LNotEqual (Local0, 0x03)))
            {
                Switch (CFML)
                {
                    Case (0x02)
                    {
                        Store (0x7530, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0x88B8, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0x927C, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }
                    Default
                    {
                        Store (0x5DC0, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0x6D60, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0x7530, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }

                }
            }
        }

        Method (AGPL, 1, Serialized)
        {
            Acquire (CMUT, 0xFFFF)
            Store (PDST, Local0)
            Release (CMUT)
            Store (Arg0, Local1)
            If (LAnd (And (Local0, 0x02), LNotEqual (Local0, 0x03)))
            {
                If (LEqual (Arg0, 0xD1))
                {
                    Store (0xD2, Local1)
                }
            }

            If (LAnd (LGreaterEqual (Local1, 0xD1), LLessEqual (Local1, 0xD4)))
            {
                Notify (^^^GPP0.PEGP, Local1)
            }
        }
    }

    Scope (_SB.PCI0.SBRG.EC0)
    {
        Name (F8FG, Zero)
        Name (BLCT, Zero)
        Name (ACNG, Zero)
        Name (DSTI, One)
        Name (DCTL, Zero)
        Name (CFML, Zero)
        Name (WRFG, Zero)
        Name (GOTF, Zero)
        Method (_Q0A, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Notify (SLPB, 0x80) // Status Change
        }

        Method (_Q0B, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Notify (ASHS, 0x88) // Device-Specific
        }

        Method (KBLD, 0, NotSerialized)
        {
            If (ATKP)
            {
                And (KBLC, 0x80, Local1)
                If (Local1)
                {
                    ^^^^ATKD.IANE (0xC5)
                }
            }
        }

        Method (KBLU, 0, NotSerialized)
        {
            If (ATKP)
            {
                And (KBLC, 0x80, Local1)
                If (Local1)
                {
                    ^^^^ATKD.IANE (0xC4)
                }
            }
        }

        Method (_Q0E, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Notify (^^^GP17.VGA.LCD, 0x87) // Device-Specific
            Return (Zero)
        }

        Method (_Q0F, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Notify (^^^GP17.VGA.LCD, 0x86) // Device-Specific
            Return (Zero)
        }

        Method (_Q10, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (LEqual (BLCT, Zero))
            {
                If (ATKP)
                {
                    ^^^^ATKD.IANE (0x33)
                }
            }
            ElseIf (LEqual (BLCT, One))
            {
                If (ATKP)
                {
                    ^^^^ATKD.IANE (0x35)
                }
            }
        }

        Method (_Q11, 0, Serialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (LEqual (F8FG, Zero))
            {
                Store (One, F8FG) /* \_SB_.PCI0.SBRG.EC0_.F8FG */
                STB2 (0xE0)
                STB2 (0x5B)
            }

            STB2 (0x19)
            STB2 (0x99)
            Return (Zero)
        }

        Method (_Q12, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (LNot (And (DSYN, One)))
            {
                If (ATKP)
                {
                    ^^^^ATKD.IANE (0x6B)
                }
            }
            ElseIf (ATKP)
            {
                ^^^^ATKD.IANE (0x6F)
            }
        }

        Method (_Q13, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            STB2 (0xE0)
            STB2 (0x20)
            STB2 (0xE0)
            STB2 (0xA0)
            Return (Zero)
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x32)
            }

            Return (Zero)
        }

        Method (_Q14, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            STB2 (0xE0)
            STB2 (0x2E)
            STB2 (0xE0)
            STB2 (0xAE)
            Return (Zero)
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x31)
            }

            Return (Zero)
        }

        Method (_Q15, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            STB2 (0xE0)
            STB2 (0x30)
            STB2 (0xE0)
            STB2 (0xB0)
            Return (Zero)
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x30)
            }

            Return (Zero)
        }

        Method (_Q6F, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x45)
            }
        }

        Method (_Q6E, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x43)
            }
        }

        Method (_Q6C, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x40)
            }
        }

        Method (_Q6D, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x41)
            }
        }

        Method (_Q78, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x7C)
            }
        }

        Method (_Q73, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (ATKP)
            {
                ^^^^ATKD.IANE (0xAE)
            }
        }

        Method (_Q72, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x8A)
            }
        }

        Method (_Q80, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (GLID ())
            {
                ST9E (0x1F, 0xFF, KLDT)
            }
            Else
            {
                Store (ST8E (0x1F, Zero), KLDT) /* \_SB_.PCI0.SBRG.EC0_.KLDT */
                ST9E (0x1F, 0xFF, Zero)
            }

            Notify (LID, 0x80) // Status Change
        }

        Method (_QAD, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (Zero, ACNG) /* \_SB_.PCI0.SBRG.EC0_.ACNG */
            If (ATKP)
            {
                ^^^^ATKD.IANE (0xCF)
            }
        }

        Method (_QAE, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (One, ACNG) /* \_SB_.PCI0.SBRG.EC0_.ACNG */
            If (ATKP)
            {
                ^^^^ATKD.IANE (0xCF)
            }
        }

        Method (_QB0, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Notify (\_TZ.THRM, 0x80) // Thermal Status Change
        }

        Method (_QA0, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Name (PDPF, Zero)
            Acquire (CMUT, 0xFFFF)
            Store (PDST, PDPF) /* \_SB_.PCI0.SBRG.EC0_._QA0.PDPF */
            Release (CMUT)
            If (ACPS ())
            {
                Store (One, ACPF) /* \_SB_.ACPF */
                Store (0x58, Local0)
                If (ATKP)
                {
                    If (LAnd (And (PDPF, 0x02), LNotEqual (PDPF, 0x03))){}
                    Else
                    {
                        ^^^^ATKD.IANE (0xCF)
                    }
                }
            }
            Else
            {
                Store (Zero, ACPF) /* \_SB_.ACPF */
                Store (0x57, Local0)
            }

            Notify (AC0, 0x80) // Status Change
            If (ATKP)
            {
                If (LAnd (And (PDPF, 0x02), LNotEqual (PDPF, 0x03))){}
                Else
                {
                    ^^^^ATKD.IANE (Local0)
                }
            }

            Sleep (0x64)
            Sleep (0x0A)
            NBAT (0x80)
        }

        Name (BLLO, Zero)
        Method (_QA1, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (DCPS (Zero), DCPF) /* \_SB_.DCPF */
            If (DCPF)
            {
                Sleep (0x07D0)
            }

            Notify (BAT0, 0x80) // Status Change
            Notify (BAT0, 0x81) // Information Change
        }

        Method (_QA5, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (One, BLLO) /* \_SB_.PCI0.SBRG.EC0_.BLLO */
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x6E)
            }
            ElseIf (BATP (Zero))
            {
                Notify (BAT0, 0x80) // Status Change
            }
        }

        Method (_QA3, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (BATP (Zero))
            {
                Store (BCLE (Zero), Local0)
                If (LEqual (Local0, Zero))
                {
                    Notify (BAT0, 0x80) // Status Change
                }
                Else
                {
                    Notify (BAT0, 0x81) // Information Change
                    Notify (AC0, 0x80) // Status Change
                }
            }
        }

        Method (_QAC, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (RP2E (0x95), Local0)
            If (LEqual (And (Local0, One), One))
            {
                Notify (BAT0, 0x80) // Status Change
            }
        }

        Method (_QB3, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            ^^^^ATKD.IANE (0x6D)
        }

        Method (_QB4, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (ATKP)
            {
                ^^^^ATKD.IANE (0xE5)
            }
        }

        Method (_QD3, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
        }

        Method (_QD4, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (LEqual (F8FG, One))
            {
                Store (Zero, F8FG) /* \_SB_.PCI0.SBRG.EC0_.F8FG */
                STB2 (0xE0)
                STB2 (0xDB)
            }
        }

        Method (_QD7, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (ATKP)
            {
                ^^^^ATKD.IANE (0xFA)
            }
        }

        Method (_QD8, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (ATKP)
            {
                ^^^^ATKD.IANE (0xFA)
            }
        }

        Method (_Q6B, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x38)
            }
        }

        Method (_QDB, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            ^^^^ATKD.IANE (0x9B)
            ^^^^ATKD.IANE (0x6D)
        }

        Method (_QDC, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            ^^^^ATKD.IANE (0x6D)
        }

        Name (CNTR, Zero)
        Method (_QDD, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            If (LEqual (CNTR, Zero))
            {
                AGPL (0xD2)
                Add (CNTR, One, CNTR) /* \_SB_.PCI0.SBRG.EC0_.CNTR */
            }
            ElseIf (LEqual (CNTR, One))
            {
                AGPL (0xD3)
                Add (CNTR, One, CNTR) /* \_SB_.PCI0.SBRG.EC0_.CNTR */
            }
            ElseIf (LEqual (CNTR, 0x02))
            {
                AGPL (0xD4)
            }
        }

        Method (_QA8, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            ^^^^ATKD.IANE (0x9A)
            AGPL (0xD4)
        }

        Method (_QA9, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            AGPL (DGD1)
            Store (Zero, CNTR) /* \_SB_.PCI0.SBRG.EC0_.CNTR */
        }

        Method (_QE8, 0, Serialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (One, DCTL) /* \_SB_.PCI0.SBRG.EC0_.DCTL */
            Name (DPTI, Buffer (0x07){})
            CreateWordField (DPTI, Zero, SSZE)
            CreateByteField (DPTI, 0x02, MSID)
            CreateDWordField (DPTI, 0x03, DECI)
            Switch (CFML)
            {
                Case (One)
                {
                    Store (0x6D60, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                    Store (0x88B8, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                    Store (0x88B8, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x5A, Local0)
                    Store (0x32, Local1)
                }
                Case (0x02)
                {
                    Store (0x61A8, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                    Store (0x927C, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                    Store (0xAAE6, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x5A, Local0)
                    Store (0x32, Local1)
                }
                Case (0x04)
                {
                    Store (0x7D00, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                    Store (0x9470, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                    Store (0xA410, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x5C, Local0)
                    Store (0x50, Local1)
                }

            }

            DCPL ()
            Store (RPIN (0x11), Local2)
            If (LEqual (Local2, Zero))
            {
                Store (0x52, Local0)
            }

            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QE8.SSZE */
            Store (0x05, MSID) /* \_SB_.PCI0.SBRG.EC0_._QE8.MSID */
            Store (PL1V, DECI) /* \_SB_.PCI0.SBRG.EC0_._QE8.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QE8.SSZE */
            Store (0x07, MSID) /* \_SB_.PCI0.SBRG.EC0_._QE8.MSID */
            Store (PL2V, DECI) /* \_SB_.PCI0.SBRG.EC0_._QE8.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QE8.SSZE */
            Store (0x06, MSID) /* \_SB_.PCI0.SBRG.EC0_._QE8.MSID */
            Store (FPTV, DECI) /* \_SB_.PCI0.SBRG.EC0_._QE8.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QE8.SSZE */
            Store (0x03, MSID) /* \_SB_.PCI0.SBRG.EC0_._QE8.MSID */
            Store (Local0, DECI) /* \_SB_.PCI0.SBRG.EC0_._QE8.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QE8.SSZE */
            Store (One, MSID) /* \_SB_.PCI0.SBRG.EC0_._QE8.MSID */
            Store (Local1, DECI) /* \_SB_.PCI0.SBRG.EC0_._QE8.DECI */
            ALIB (0x0C, DPTI)
            If (LAnd (LEqual (GOTF, Zero), LEqual (CNTR, Zero)))
            {
                AGPL (DGD1)
            }
        }

        Method (_QF1, 0, Serialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (Zero, DCTL) /* \_SB_.PCI0.SBRG.EC0_.DCTL */
            Name (DPTI, Buffer (0x07){})
            CreateWordField (DPTI, Zero, SSZE)
            CreateByteField (DPTI, 0x02, MSID)
            CreateDWordField (DPTI, 0x03, DECI)
            Switch (CFML)
            {
                Case (One)
                {
                    If (LEqual (CPFG, 0x02))
                    {
                        Store (0x88B8, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0xA410, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0xEA60, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }
                    Else
                    {
                        Store (0x88B8, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0xD2F0, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0xFDE8, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }

                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x5E, Local0)
                    Store (0x96, Local1)
                }
                Case (0x02)
                {
                    Store (0x7A12, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                    Store (0xAAE6, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                    Store (0xF424, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x5A, Local0)
                    Store (0x3C, Local1)
                }
                Case (0x04)
                {
                    If (LEqual (CPFG, 0x02))
                    {
                        Store (0x88B8, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0xA410, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0xEA60, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }
                    Else
                    {
                        Store (0x88B8, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0xD2F0, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0xFDE8, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }

                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x60, Local0)
                    Store (0xC8, Local1)
                }

            }

            DCPL ()
            Store (RPIN (0x11), Local2)
            If (LEqual (Local2, Zero))
            {
                Store (0x52, Local0)
            }

            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QF1.SSZE */
            Store (0x05, MSID) /* \_SB_.PCI0.SBRG.EC0_._QF1.MSID */
            Store (PL1V, DECI) /* \_SB_.PCI0.SBRG.EC0_._QF1.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QF1.SSZE */
            Store (0x07, MSID) /* \_SB_.PCI0.SBRG.EC0_._QF1.MSID */
            Store (PL2V, DECI) /* \_SB_.PCI0.SBRG.EC0_._QF1.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QF1.SSZE */
            Store (0x06, MSID) /* \_SB_.PCI0.SBRG.EC0_._QF1.MSID */
            Store (FPTV, DECI) /* \_SB_.PCI0.SBRG.EC0_._QF1.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QF1.SSZE */
            Store (0x03, MSID) /* \_SB_.PCI0.SBRG.EC0_._QF1.MSID */
            Store (Local0, DECI) /* \_SB_.PCI0.SBRG.EC0_._QF1.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QF1.SSZE */
            Store (One, MSID) /* \_SB_.PCI0.SBRG.EC0_._QF1.MSID */
            Store (Local1, DECI) /* \_SB_.PCI0.SBRG.EC0_._QF1.DECI */
            ALIB (0x0C, DPTI)
            If (LAnd (LEqual (GOTF, Zero), LEqual (CNTR, Zero)))
            {
                AGPL (DGD1)
            }

            Store (Zero, WRFG) /* \_SB_.PCI0.SBRG.EC0_.WRFG */
        }

        Method (_QF0, 0, Serialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (Zero, DCTL) /* \_SB_.PCI0.SBRG.EC0_.DCTL */
            Name (DPTI, Buffer (0x07){})
            CreateWordField (DPTI, Zero, SSZE)
            CreateByteField (DPTI, 0x02, MSID)
            CreateDWordField (DPTI, 0x03, DECI)
            Switch (CFML)
            {
                Case (One)
                {
                    If (LEqual (CPFG, 0x02))
                    {
                        Store (0x7530, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0x9C40, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0xD6D8, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }
                    Else
                    {
                        Store (0x7530, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0x9C40, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0xD6D8, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }

                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x5A, Local0)
                    Store (0x64, Local1)
                }
                Case (0x02)
                {
                    Store (0x7A12, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                    Store (0xAAE6, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                    Store (0xF424, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x5A, Local0)
                    Store (0x3C, Local1)
                }
                Case (0x04)
                {
                    If (LEqual (CPFG, 0x02))
                    {
                        Store (0x88B8, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0xA410, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0xEA60, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }
                    Else
                    {
                        Store (0x88B8, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                        Store (0xD2F0, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                        Store (0xFDE8, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    }

                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x60, Local0)
                    Store (0x96, Local1)
                }

            }

            DCPL ()
            Store (RPIN (0x11), Local2)
            If (LEqual (Local2, Zero))
            {
                Store (0x52, Local0)
            }

            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QF0.SSZE */
            Store (0x05, MSID) /* \_SB_.PCI0.SBRG.EC0_._QF0.MSID */
            Store (PL1V, DECI) /* \_SB_.PCI0.SBRG.EC0_._QF0.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QF0.SSZE */
            Store (0x07, MSID) /* \_SB_.PCI0.SBRG.EC0_._QF0.MSID */
            Store (PL2V, DECI) /* \_SB_.PCI0.SBRG.EC0_._QF0.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QF0.SSZE */
            Store (0x06, MSID) /* \_SB_.PCI0.SBRG.EC0_._QF0.MSID */
            Store (FPTV, DECI) /* \_SB_.PCI0.SBRG.EC0_._QF0.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QF0.SSZE */
            Store (0x03, MSID) /* \_SB_.PCI0.SBRG.EC0_._QF0.MSID */
            Store (Local0, DECI) /* \_SB_.PCI0.SBRG.EC0_._QF0.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QF0.SSZE */
            Store (One, MSID) /* \_SB_.PCI0.SBRG.EC0_._QF0.MSID */
            Store (Local1, DECI) /* \_SB_.PCI0.SBRG.EC0_._QF0.DECI */
            ALIB (0x0C, DPTI)
            If (LAnd (LEqual (GOTF, Zero), LEqual (CNTR, Zero)))
            {
                AGPL (DGD1)
            }

            Store (One, WRFG) /* \_SB_.PCI0.SBRG.EC0_.WRFG */
        }

        Method (_QEA, 0, Serialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (0x02, DCTL) /* \_SB_.PCI0.SBRG.EC0_.DCTL */
            Name (DPTI, Buffer (0x07){})
            CreateWordField (DPTI, Zero, SSZE)
            CreateByteField (DPTI, 0x02, MSID)
            CreateDWordField (DPTI, 0x03, DECI)
            Switch (CFML)
            {
                Case (One)
                {
                    Store (0x59D8, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                    Store (0x61A8, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                    Store (0x61A8, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x58, Local0)
                    Store (0x32, Local1)
                }
                Case (0x02)
                {
                    Store (0x61A8, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                    Store (0x7A12, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                    Store (0x927C, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x58, Local0)
                    Store (0x32, Local1)
                }
                Case (0x04)
                {
                    Store (0x6D60, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                    Store (0x88B8, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                    Store (0x9C40, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x5A, Local0)
                    Store (0x32, Local1)
                }

            }

            DCPL ()
            Store (RPIN (0x11), Local2)
            If (LEqual (Local2, Zero))
            {
                Store (0x52, Local0)
            }

            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QEA.SSZE */
            Store (0x05, MSID) /* \_SB_.PCI0.SBRG.EC0_._QEA.MSID */
            Store (PL1V, DECI) /* \_SB_.PCI0.SBRG.EC0_._QEA.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QEA.SSZE */
            Store (0x07, MSID) /* \_SB_.PCI0.SBRG.EC0_._QEA.MSID */
            Store (PL2V, DECI) /* \_SB_.PCI0.SBRG.EC0_._QEA.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QEA.SSZE */
            Store (0x06, MSID) /* \_SB_.PCI0.SBRG.EC0_._QEA.MSID */
            Store (FPTV, DECI) /* \_SB_.PCI0.SBRG.EC0_._QEA.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QEA.SSZE */
            Store (0x03, MSID) /* \_SB_.PCI0.SBRG.EC0_._QEA.MSID */
            Store (Local0, DECI) /* \_SB_.PCI0.SBRG.EC0_._QEA.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QEA.SSZE */
            Store (One, MSID) /* \_SB_.PCI0.SBRG.EC0_._QEA.MSID */
            Store (Local1, DECI) /* \_SB_.PCI0.SBRG.EC0_._QEA.DECI */
            ALIB (0x0C, DPTI)
            If (LAnd (LEqual (GOTF, Zero), LEqual (CNTR, Zero)))
            {
                AGPL (DGD1)
            }
        }

        Method (_QEB, 0, Serialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (One, DCTL) /* \_SB_.PCI0.SBRG.EC0_.DCTL */
            Name (DPTI, Buffer (0x07){})
            CreateWordField (DPTI, Zero, SSZE)
            CreateByteField (DPTI, 0x02, MSID)
            CreateDWordField (DPTI, 0x03, DECI)
            Switch (CFML)
            {
                Case (One)
                {
                    Store (0x6D60, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                    Store (0x88B8, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                    Store (0x88B8, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x5A, Local0)
                    Store (0x32, Local1)
                }
                Case (0x02)
                {
                    Store (0x61A8, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                    Store (0x927C, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                    Store (0xAAE6, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x5A, Local0)
                    Store (0x32, Local1)
                }
                Case (0x04)
                {
                    Store (0x7D00, PL1V) /* \_SB_.PCI0.SBRG.EC0_.PL1V */
                    Store (0x9470, PL2V) /* \_SB_.PCI0.SBRG.EC0_.PL2V */
                    Store (0xA410, FPTV) /* \_SB_.PCI0.SBRG.EC0_.FPTV */
                    Store (0xD1, DGD1) /* \_SB_.PCI0.SBRG.EC0_.DGD1 */
                    Store (0x5C, Local0)
                    Store (0x50, Local1)
                }

            }

            DCPL ()
            Store (RPIN (0x11), Local2)
            If (LEqual (Local2, Zero))
            {
                Store (0x52, Local0)
            }

            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QEB.SSZE */
            Store (0x05, MSID) /* \_SB_.PCI0.SBRG.EC0_._QEB.MSID */
            Store (PL1V, DECI) /* \_SB_.PCI0.SBRG.EC0_._QEB.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QEB.SSZE */
            Store (0x07, MSID) /* \_SB_.PCI0.SBRG.EC0_._QEB.MSID */
            Store (PL2V, DECI) /* \_SB_.PCI0.SBRG.EC0_._QEB.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QEB.SSZE */
            Store (0x06, MSID) /* \_SB_.PCI0.SBRG.EC0_._QEB.MSID */
            Store (FPTV, DECI) /* \_SB_.PCI0.SBRG.EC0_._QEB.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QEB.SSZE */
            Store (0x03, MSID) /* \_SB_.PCI0.SBRG.EC0_._QEB.MSID */
            Store (Local0, DECI) /* \_SB_.PCI0.SBRG.EC0_._QEB.DECI */
            ALIB (0x0C, DPTI)
            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QEB.SSZE */
            Store (One, MSID) /* \_SB_.PCI0.SBRG.EC0_._QEB.MSID */
            Store (Local1, DECI) /* \_SB_.PCI0.SBRG.EC0_._QEB.DECI */
            ALIB (0x0C, DPTI)
            If (LAnd (LEqual (GOTF, Zero), LEqual (CNTR, Zero)))
            {
                AGPL (DGD1)
            }
        }

        Method (_QEC, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Name (DPTI, Buffer (0x07){})
            CreateWordField (DPTI, Zero, SSZE)
            CreateByteField (DPTI, 0x02, MSID)
            CreateDWordField (DPTI, 0x03, DECI)
            Store (RPIN (0x11), Local0)
            If (LEqual (Local0, Zero))
            {
                Store (0x52, Local1)
            }
            Else
            {
                Store (0x64, Local1)
            }

            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QEC.SSZE */
            Store (0x03, MSID) /* \_SB_.PCI0.SBRG.EC0_._QEC.MSID */
            Store (Local1, DECI) /* \_SB_.PCI0.SBRG.EC0_._QEC.DECI */
            ALIB (0x0C, DPTI)
        }

        Method (_QED, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Name (DPTI, Buffer (0x07){})
            CreateWordField (DPTI, Zero, SSZE)
            CreateByteField (DPTI, 0x02, MSID)
            CreateDWordField (DPTI, 0x03, DECI)
            Store (RPIN (0x11), Local0)
            If (LEqual (Local0, Zero))
            {
                Store (0x52, Local1)
            }
            Else
            {
                Store (0x5F, Local1)
            }

            Store (0x07, SSZE) /* \_SB_.PCI0.SBRG.EC0_._QED.SSZE */
            Store (0x03, MSID) /* \_SB_.PCI0.SBRG.EC0_._QED.MSID */
            Store (Local1, DECI) /* \_SB_.PCI0.SBRG.EC0_._QED.DECI */
            ALIB (0x0C, DPTI)
        }

        Method (_QEF, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            ^^^^ATKD.IANE (0xE8)
        }

        Method (_QAF, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (Acquire (CMUT, 0xFFFF), Local1)
            If (LEqual (Local1, Zero))
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (MGI0, ^^^^UBTC.MGI0) /* External reference */
                Store (MGI1, ^^^^UBTC.MGI1) /* External reference */
                Store (MGI2, ^^^^UBTC.MGI2) /* External reference */
                Store (MGI3, ^^^^UBTC.MGI3) /* External reference */
                Store (MGI4, ^^^^UBTC.MGI4) /* External reference */
                Store (MGI5, ^^^^UBTC.MGI5) /* External reference */
                Store (MGI6, ^^^^UBTC.MGI6) /* External reference */
                Store (MGI7, ^^^^UBTC.MGI7) /* External reference */
                Store (MGI8, ^^^^UBTC.MGI8) /* External reference */
                Store (MGI9, ^^^^UBTC.MGI9) /* External reference */
                Store (MGIA, ^^^^UBTC.MGIA) /* External reference */
                Store (MGIB, ^^^^UBTC.MGIB) /* External reference */
                Store (MGIC, ^^^^UBTC.MGIC) /* External reference */
                Store (MGID, ^^^^UBTC.MGID) /* External reference */
                Store (MGIE, ^^^^UBTC.MGIE) /* External reference */
                Store (MGIF, ^^^^UBTC.MGIF) /* External reference */
                Store (CCI0, ^^^^UBTC.CCI0) /* External reference */
                Store (CCI1, ^^^^UBTC.CCI1) /* External reference */
                Store (CCI2, ^^^^UBTC.CCI2) /* External reference */
                Store (CCI3, ^^^^UBTC.CCI3) /* External reference */
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Release (CMUT)
            }

            Notify (UBTC, 0x80) // Status Change
        }

        Method (_QD5, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
        }

        Method (_QD9, 0, Serialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Store (STCC (Zero, 0x14), CFML) /* \_SB_.PCI0.SBRG.EC0_.CFML */
            Switch (ToInteger (DCTL))
            {
                Case (Zero)
                {
                    If (LEqual (WRFG, One))
                    {
                        _QF0 ()
                    }
                    Else
                    {
                        _QF1 ()
                    }
                }
                Case (One)
                {
                    _QE8 ()
                }
                Case (0x02)
                {
                    _QEA ()
                }

            }
        }

        Method (_QC7, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            ^^^^ATKD.IANE (0x6D)
        }

        Method (_QEE, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Notify (^^^GPP0.PEGP, 0x81) // Information Change
        }
    }

    Scope (_SB.PCI0.SBRG.EC0)
    {
        Method (CHGS, 1, Serialized)
        {
            Store (BCHG (Arg0), Local0)
            Return (Local0)
        }

        Name (BSLF, Zero)
        Method (BATS, 1, Serialized)
        {
            If (Arg0)
            {
                Store (One, BSLF) /* \_SB_.PCI0.SBRG.EC0_.BSLF */
            }
            Else
            {
                Store (Zero, BSLF) /* \_SB_.PCI0.SBRG.EC0_.BSLF */
            }
        }

        Mutex (MMPX, 0x00)
        Method (BATO, 0, Serialized)
        {
            Acquire (MMPX, 0xFFFF)
        }

        Method (BATR, 0, Serialized)
        {
            Release (MMPX)
        }

        Method (BATP, 1, Serialized)
        {
            If (Arg0)
            {
                Return (DCP2) /* \_SB_.DCP2 */
            }
            Else
            {
                Return (DCPF) /* \_SB_.DCPF */
            }
        }

        Method (NBAT, 1, NotSerialized)
        {
            If (BATP (Zero))
            {
                Notify (BAT0, Arg0)
            }
        }
    }

    Scope (_SB.PCI0.SBRG.EC0)
    {
        Name (BADR, 0x0B)
        Name (CADR, 0x09)
        Name (SADR, 0x0A)
        Method (BIF0, 0, NotSerialized)
        {
            If (ECAV ())
            {
                Acquire (CMUT, 0xFFFF)
                If (BSLF)
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (B1MD, Local0)
                }
                Else
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (B0MD, Local0)
                }

                Release (CMUT)
                If (LNotEqual (Local0, 0xFFFF))
                {
                    ShiftRight (Local0, 0x0F, Local1)
                    And (Local1, One, Local1)
                    XOr (Local1, One, Local0)
                }
            }
            Else
            {
                Store (Ones, Local0)
            }

            Return (Local0)
        }

        Method (BIF1, 0, NotSerialized)
        {
            If (ECAV ())
            {
                Acquire (CMUT, 0xFFFF)
                If (BSLF)
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (B1DC, Local0)
                }
                Else
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (B0DC, Local0)
                }

                Release (CMUT)
                And (Local0, 0xFFFF, Local0)
            }
            Else
            {
                Store (Ones, Local0)
            }

            Return (Local0)
        }

        Method (BIF2, 0, NotSerialized)
        {
            If (ECAV ())
            {
                Acquire (CMUT, 0xFFFF)
                If (BSLF)
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (B1FC, Local0)
                }
                Else
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (B0FC, Local0)
                }

                Release (CMUT)
                And (Local0, 0xFFFF, Local0)
            }
            Else
            {
                Store (Ones, Local0)
            }

            Return (Local0)
        }

        Method (BIF3, 0, NotSerialized)
        {
            If (ECAV ())
            {
                Acquire (CMUT, 0xFFFF)
                If (BSLF)
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (B1MD, Local0)
                }
                Else
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (B0MD, Local0)
                }

                Release (CMUT)
                If (LNotEqual (Local0, 0xFFFF))
                {
                    ShiftRight (Local0, 0x09, Local0)
                    And (Local0, One, Local0)
                    XOr (Local0, One, Local0)
                }
            }
            Else
            {
                Store (Ones, Local0)
            }

            Return (Local0)
        }

        Method (BIF4, 0, NotSerialized)
        {
            If (ECAV ())
            {
                Acquire (CMUT, 0xFFFF)
                If (BSLF)
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (B1DV, Local0)
                }
                Else
                {
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (B0DV, Local0)
                }

                Release (CMUT)
            }
            Else
            {
                Store (Ones, Local0)
            }

            Return (Local0)
        }

        Method (BIF9, 0, NotSerialized)
        {
            Name (BSTR, Buffer (0x20){})
            Name (DDDD, "ASUS Battery")
            Store (DDDD, BSTR) /* \_SB_.PCI0.SBRG.EC0_.BIF9.BSTR */
            Return (BSTR) /* \_SB_.PCI0.SBRG.EC0_.BIF9.BSTR */
        }

        Method (BSTS, 0, NotSerialized)
        {
            Acquire (CMUT, 0xFFFF)
            If (BSLF)
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (B1ST, Local0)
            }
            Else
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (B0ST, Local0)
            }

            Release (CMUT)
            Return (Local0)
        }

        Method (BCRT, 0, NotSerialized)
        {
            Acquire (CMUT, 0xFFFF)
            If (BSLF)
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (B1CC, Local0)
            }
            Else
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (B0CC, Local0)
            }

            Release (CMUT)
            Return (Local0)
        }

        Method (BRCP, 0, NotSerialized)
        {
            Acquire (CMUT, 0xFFFF)
            If (BSLF)
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (B1RC, Local0)
            }
            Else
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (B0RC, Local0)
            }

            Release (CMUT)
            If (LEqual (Local0, 0xFFFF))
            {
                Store (Ones, Local0)
            }

            Return (Local0)
        }

        Method (BVOT, 0, NotSerialized)
        {
            Acquire (CMUT, 0xFFFF)
            If (BSLF)
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (B1VL, Local0)
            }
            Else
            {
                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                Store (B0VL, Local0)
            }

            Release (CMUT)
            Return (Local0)
        }
    }

    Scope (_SB.PCI0.SBRG.EC0)
    {
        Device (BAT0)
        {
            Name (_HID, EisaId ("PNP0C0A") /* Control Method Battery */)  // _HID: Hardware ID
            Name (_UID, Zero)  // _UID: Unique ID
            Name (_PCL, Package (0x01)  // _PCL: Power Consumer List
            {
                PCI0
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (BATP (Zero))
                {
                    Return (0x1F)
                }
                Else
                {
                    Return (0x0F)
                }
            }

            Name (LOW2, 0x012C)
            Name (PUNT, One)
            Name (LFCC, 0x1770)
            Name (NBIF, Package (0x0D)
            {
                Zero, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                One, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                "", 
                "", 
                "", 
                ""
            })
            Name (PBIF, Package (0x0D)
            {
                One, 
                0x1770, 
                0x1770, 
                One, 
                0x39D0, 
                0x0258, 
                0x012C, 
                0x3C, 
                0x3C, 
                "M3N", 
                " ", 
                "LIon", 
                "ASUSTeK"
            })
            Name (PBST, Package (0x04)
            {
                Zero, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF
            })
            Name (BIXT, Package (0x14)
            {
                Zero, 
                Zero, 
                0x1770, 
                0x1770, 
                One, 
                0x39D0, 
                0x0258, 
                0x012C, 
                Zero, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0x3C, 
                0x3C, 
                "M3N", 
                " ", 
                "LIon", 
                "ASUSTeK"
            })
            Name (NBIX, Package (0x14)
            {
                Zero, 
                Zero, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                One, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                Zero, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                0xFFFFFFFF, 
                "", 
                "", 
                "", 
                ""
            })
            Method (FBIF, 5, NotSerialized)
            {
                Store (Arg0, PUNT) /* \_SB_.PCI0.SBRG.EC0_.BAT0.PUNT */
                Store (Arg1, Local1)
                Store (Arg2, Local2)
                If (LEqual (PUNT, Zero))
                {
                    Multiply (Local1, 0x0A, Local1)
                    Multiply (Local2, 0x0A, Local2)
                }

                Store (Arg0, Index (PBIF, Zero))
                Store (Local1, Index (PBIF, One))
                Store (Local2, Index (PBIF, 0x02))
                Store (Local2, LFCC) /* \_SB_.PCI0.SBRG.EC0_.BAT0.LFCC */
                Store (Arg3, Index (PBIF, 0x03))
                Store (Arg4, Index (PBIF, 0x04))
                Divide (Local1, 0x0A, , Local5)
                Store (Local5, Index (PBIF, 0x05))
                Divide (Local1, 0x64, , Local6)
                Store (Local6, Index (PBIF, 0x06))
                Store (Local6, LOW2) /* \_SB_.PCI0.SBRG.EC0_.BAT0.LOW2 */
                Divide (Local1, 0x64, , Local7)
                Store (Local7, Index (PBIF, 0x07))
                Store (Local7, Index (PBIF, 0x08))
            }

            Method (CBIF, 0, NotSerialized)
            {
                If (PUNT)
                {
                    Store (DerefOf (Index (PBIF, 0x04)), Local0)
                    Store (Zero, Index (PBIF, Zero))
                    Multiply (DerefOf (Index (PBIF, One)), Local0, Index (PBIF, One))
                    Multiply (DerefOf (Index (PBIF, 0x02)), Local0, Index (PBIF, 0x02))
                    Multiply (DerefOf (Index (PBIF, 0x05)), Local0, Index (PBIF, 0x05))
                    Multiply (DerefOf (Index (PBIF, 0x06)), Local0, Index (PBIF, 0x06))
                    Multiply (DerefOf (Index (PBIF, 0x07)), Local0, Index (PBIF, 0x07))
                    Multiply (DerefOf (Index (PBIF, 0x08)), Local0, Index (PBIF, 0x08))
                    Divide (DerefOf (Index (PBIF, One)), 0x03E8, , Index (PBIF, One))
                    Divide (DerefOf (Index (PBIF, 0x02)), 0x03E8, , Index (PBIF, 0x02))
                    Divide (DerefOf (Index (PBIF, 0x05)), 0x03E8, , Index (PBIF, 0x05))
                    Divide (DerefOf (Index (PBIF, 0x06)), 0x03E8, , Index (PBIF, 0x06))
                    Divide (DerefOf (Index (PBIF, 0x07)), 0x03E8, , Index (PBIF, 0x07))
                    Divide (DerefOf (Index (PBIF, 0x08)), 0x03E8, , Index (PBIF, 0x08))
                }
            }

            Method (_BIF, 0, NotSerialized)  // _BIF: Battery Information
            {
                If (LNot (BATP (Zero)))
                {
                    Return (NBIF) /* \_SB_.PCI0.SBRG.EC0_.BAT0.NBIF */
                }

                If (LEqual (GBTT (Zero), 0xFF))
                {
                    Return (NBIF) /* \_SB_.PCI0.SBRG.EC0_.BAT0.NBIF */
                }

                BATO ()
                BATS (Zero)
                Store (BIF9 (), Index (PBIF, 0x09))
                Store (BIF0 (), Local0)
                Store (BIF1 (), Local1)
                Store (BIF2 (), Local2)
                Store (BIF3 (), Local3)
                Store (BIF4 (), Local4)
                If (LNotEqual (Local0, Ones))
                {
                    If (LNotEqual (Local1, Ones))
                    {
                        If (LNotEqual (Local2, Ones))
                        {
                            If (LNotEqual (Local3, Ones))
                            {
                                If (LNotEqual (Local4, Ones))
                                {
                                    FBIF (Local0, Local1, Local2, Local3, Local4)
                                    CBIF ()
                                }
                            }
                        }
                    }
                }

                If (LEqual (PUNT, Zero))
                {
                    Multiply (Local2, 0x0A, Local2)
                }

                Store (Local2, LFCC) /* \_SB_.PCI0.SBRG.EC0_.BAT0.LFCC */
                BATR ()
                Return (PBIF) /* \_SB_.PCI0.SBRG.EC0_.BAT0.PBIF */
            }

            Method (FBST, 4, NotSerialized)
            {
                And (Arg1, 0xFFFF, Local1)
                Store (Zero, Local0)
                If (ACAP ())
                {
                    Store (One, Local0)
                }

                If (Local0)
                {
                    If (CHGS (Zero))
                    {
                        Store (0x02, Local0)
                    }
                    Else
                    {
                        Store (One, Local0)
                    }
                }
                Else
                {
                    Store (One, Local0)
                }

                If (BLLO)
                {
                    ShiftLeft (One, 0x02, Local2)
                    Or (Local0, Local2, Local0)
                }

                Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                If (And (EB0S, 0x08))
                {
                    ShiftLeft (One, 0x02, Local2)
                    Or (Local0, Local2, Local0)
                }

                If (LGreaterEqual (Local1, 0x8000))
                {
                    Subtract (0xFFFF, Local1, Local1)
                }

                Store (Arg2, Local2)
                If (LEqual (PUNT, Zero))
                {
                    Acquire (CMUT, 0xFFFF)
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Multiply (Local1, B0DV, Local1)
                    Release (CMUT)
                    Multiply (Local2, 0x0A, Local2)
                }

                And (Local0, 0x02, Local3)
                If (LNot (Local3))
                {
                    Subtract (LFCC, Local2, Local3)
                    Divide (LFCC, 0xC8, Local4, Local5)
                    If (LLess (Local3, Local5))
                    {
                        Store (LFCC, Local2)
                    }
                }
                Else
                {
                    Divide (LFCC, 0xC8, Local4, Local5)
                    Subtract (LFCC, Local5, Local4)
                    If (LGreater (Local2, Local4))
                    {
                        Store (Local4, Local2)
                    }
                }

                If (LNot (ACAP ()))
                {
                    Divide (Local2, MBLF, Local3, Local4)
                    If (LLess (Local1, Local4))
                    {
                        Store (Local4, Local1)
                    }
                }

                Store (Local0, Index (PBST, Zero))
                Store (Local1, Index (PBST, One))
                Store (Local2, Index (PBST, 0x02))
                Store (Arg3, Index (PBST, 0x03))
            }

            Method (CBST, 0, NotSerialized)
            {
                If (PUNT)
                {
                    Acquire (CMUT, 0xFFFF)
                    Store (0xC9, BRAH) /* \_SB_.PCI0.SBRG.EC0_.BRAH */
                    Store (B0DV, Index (PBST, 0x03))
                    Release (CMUT)
                    Store (DerefOf (Index (PBST, 0x03)), Local0)
                    Multiply (DerefOf (Index (PBST, One)), Local0, Index (PBST, One))
                    Divide (DerefOf (Index (PBST, One)), 0x03E8, , Index (PBST, One))
                    Multiply (DerefOf (Index (PBST, 0x02)), Local0, Index (PBST, 0x02))
                    Divide (DerefOf (Index (PBST, 0x02)), 0x03E8, , Index (PBST, 0x02))
                }
            }

            Method (_BST, 0, NotSerialized)  // _BST: Battery Status
            {
                Store (Zero, Index (PBST, Zero))
                Store (Ones, Index (PBST, One))
                Store (Ones, Index (PBST, 0x02))
                Store (Ones, Index (PBST, 0x03))
                If (LNot (BATP (Zero)))
                {
                    Store (One, Index (PBST, Zero))
                    Return (PBST) /* \_SB_.PCI0.SBRG.EC0_.BAT0.PBST */
                }

                If (LEqual (GBTT (Zero), 0xFF))
                {
                    Return (PBST) /* \_SB_.PCI0.SBRG.EC0_.BAT0.PBST */
                }

                BATO ()
                BATS (Zero)
                Store (BSTS (), Local0)
                Store (BCRT (), Local1)
                Store (BRCP (), Local2)
                Store (BVOT (), Local3)
                If (LNotEqual (Local0, Ones))
                {
                    If (LNotEqual (Local1, Ones))
                    {
                        If (LNotEqual (Local2, Ones))
                        {
                            If (LNotEqual (Local3, Ones))
                            {
                                FBST (Local0, Local1, Local2, Local3)
                                CBST ()
                            }
                        }
                    }
                }

                BATR ()
                Return (PBST) /* \_SB_.PCI0.SBRG.EC0_.BAT0.PBST */
            }

            Method (_BIX, 0, NotSerialized)  // _BIX: Battery Information Extended
            {
                If (LNot (BATP (Zero)))
                {
                    Return (NBIX) /* \_SB_.PCI0.SBRG.EC0_.BAT0.NBIX */
                }

                If (LEqual (GBTT (Zero), 0xFF))
                {
                    Return (NBIX) /* \_SB_.PCI0.SBRG.EC0_.BAT0.NBIX */
                }

                _BIF ()
                Store (DerefOf (Index (PBIF, Zero)), Index (BIXT, One))
                Store (DerefOf (Index (PBIF, One)), Index (BIXT, 0x02))
                Store (DerefOf (Index (PBIF, 0x02)), Index (BIXT, 0x03))
                Store (DerefOf (Index (PBIF, 0x03)), Index (BIXT, 0x04))
                Store (DerefOf (Index (PBIF, 0x04)), Index (BIXT, 0x05))
                Store (DerefOf (Index (PBIF, 0x05)), Index (BIXT, 0x06))
                Store (DerefOf (Index (PBIF, 0x06)), Index (BIXT, 0x07))
                Store (DerefOf (Index (PBIF, 0x07)), Index (BIXT, 0x0E))
                Store (DerefOf (Index (PBIF, 0x08)), Index (BIXT, 0x0F))
                Store (DerefOf (Index (PBIF, 0x09)), Index (BIXT, 0x10))
                Store (DerefOf (Index (PBIF, 0x0A)), Index (BIXT, 0x11))
                Store (DerefOf (Index (PBIF, 0x0B)), Index (BIXT, 0x12))
                Store (DerefOf (Index (PBIF, 0x0C)), Index (BIXT, 0x13))
                If (LEqual (DerefOf (Index (BIXT, One)), One))
                {
                    Store (Zero, Index (BIXT, One))
                    Store (DerefOf (Index (BIXT, 0x05)), Local0)
                    Multiply (DerefOf (Index (BIXT, 0x02)), Local0, Index (BIXT, 0x02))
                    Multiply (DerefOf (Index (BIXT, 0x03)), Local0, Index (BIXT, 0x03))
                    Multiply (DerefOf (Index (BIXT, 0x06)), Local0, Index (BIXT, 0x06))
                    Multiply (DerefOf (Index (BIXT, 0x07)), Local0, Index (BIXT, 0x07))
                    Multiply (DerefOf (Index (BIXT, 0x0E)), Local0, Index (BIXT, 0x0E))
                    Multiply (DerefOf (Index (BIXT, 0x0F)), Local0, Index (BIXT, 0x0F))
                    Divide (DerefOf (Index (BIXT, 0x02)), 0x03E8, Local0, Index (BIXT, 0x02))
                    Divide (DerefOf (Index (BIXT, 0x03)), 0x03E8, Local0, Index (BIXT, 0x03))
                    Divide (DerefOf (Index (BIXT, 0x06)), 0x03E8, Local0, Index (BIXT, 0x06))
                    Divide (DerefOf (Index (BIXT, 0x07)), 0x03E8, Local0, Index (BIXT, 0x07))
                    Divide (DerefOf (Index (BIXT, 0x0E)), 0x03E8, Local0, Index (BIXT, 0x0E))
                    Divide (DerefOf (Index (BIXT, 0x0F)), 0x03E8, Local0, Index (BIXT, 0x0F))
                }

                Store (0x0001869F, Index (BIXT, 0x09))
                Return (BIXT) /* \_SB_.PCI0.SBRG.EC0_.BAT0.BIXT */
            }

            Method (_BTP, 1, NotSerialized)  // _BTP: Battery Trip Point
            {
                RP2E (0x96)
                RP2E (0x97)
                RP2E (0x95)
                If (LNotEqual (Arg0, Zero))
                {
                    Store (DerefOf (Index (PBIF, 0x04)), Local0)
                    Multiply (Arg0, 0x03E8, Local1)
                    Divide (Local1, Local0, Local2, Local3)
                    If (CHGS (Zero))
                    {
                        If (LNotEqual (Local2, Zero))
                        {
                            Add (Local3, One, Local3)
                        }
                    }

                    WP2E (0x96, And (Local3, 0xFF))
                    ShiftRight (Local3, 0x08, Local2)
                    And (Local2, 0xFF, Local2)
                    WP2E (0x97, Local2)
                    WP2E (0x95, Zero)
                }
            }
        }
    }

    Scope (\)
    {
        Name (TSP, 0x0A)
        Name (TC1, 0x02)
        Name (TC2, 0x0A)
    }

    Scope (_TZ)
    {
        Name (ETMD, One)
        Method (KELV, 1, NotSerialized)
        {
            And (Arg0, 0xFF, Local0)
            If (LGreaterEqual (Local0, 0x80))
            {
                Subtract (0x0100, Local0, Local0)
                Multiply (Local0, 0x0A, Local0)
                Subtract (0x0AAC, Local0, Local0)
                Return (Local0)
            }

            Multiply (Local0, 0x0A, Local0)
            Add (Local0, 0x0AAC, Local0)
            Return (Local0)
        }

        Method (CELC, 1, NotSerialized)
        {
            Subtract (Arg0, 0x0AAC, Local0)
            Divide (Local0, 0x0A, , Local0)
            Return (Local0)
        }

        Name (PLCY, Zero)
        ThermalZone (THRM)
        {
            Method (_CRT, 0, NotSerialized)  // _CRT: Critical Temperature
            {
                RCRT ()
                Return (KELV (\_SB.TCRT))
            }

            Method (_TMP, 0, NotSerialized)  // _TMP: Temperature
            {
                Store (0x05, Local1)
                While (Local1)
                {
                    Store (RTMP (), Local0)
                    If (LGreater (Local0, \_SB.TCRT))
                    {
                        Decrement (Local1)
                    }
                    Else
                    {
                        Store (Zero, Local1)
                    }
                }

                Return (KELV (Local0))
            }

            Name (_PSL, Package (0x10)  // _PSL: Passive List
            {
                \_SB.PLTF.P000, 
                \_SB.PLTF.P001, 
                \_SB.PLTF.P002, 
                \_SB.PLTF.P003, 
                \_SB.PLTF.P004, 
                \_SB.PLTF.P005, 
                \_SB.PLTF.P006, 
                \_SB.PLTF.P007, 
                \_SB.PLTF.P008, 
                \_SB.PLTF.P009, 
                \_SB.PLTF.P00A, 
                \_SB.PLTF.P00B, 
                \_SB.PLTF.P00C, 
                \_SB.PLTF.P00D, 
                \_SB.PLTF.P00E, 
                \_SB.PLTF.P00F
            })
            Method (_TSP, 0, NotSerialized)  // _TSP: Thermal Sampling Period
            {
                Multiply (TSP, 0x0A, Local0)
                Return (Local0)
            }

            Method (_TC1, 0, NotSerialized)  // _TC1: Thermal Constant 1
            {
                Return (TC1) /* \TC1_ */
            }

            Method (_TC2, 0, NotSerialized)  // _TC2: Thermal Constant 2
            {
                Return (TC2) /* \TC2_ */
            }

            Method (_PSV, 0, NotSerialized)  // _PSV: Passive Temperature
            {
                RPSV ()
                If (PLCY)
                {
                    Return (KELV (\_SB.PPSV))
                }
                Else
                {
                    Return (KELV (\_SB.TPSV))
                }
            }

            Method (_SCP, 1, NotSerialized)  // _SCP: Set Cooling Policy
            {
                Store (Zero, PLCY) /* \_TZ_.PLCY */
                Notify (THRM, 0x81) // Thermal Trip Point Change
            }
        }
    }

    Scope (_TZ)
    {
        Name (ATMP, 0x3C)
        Name (LTMP, 0x3C)
        Name (FANS, Zero)
        Method (RTMP, 0, NotSerialized)
        {
            If (\_SB.PCI0.SBRG.EC0.ECAV ())
            {
                Acquire (\_SB.PCI0.SBRG.EC0.CMUT, 0xFFFF)
                Store (0xC9, \_SB.PCI0.SBRG.EC0.BRAH)
                Store (\_SB.PCI0.SBRG.EC0.ECPU, Local0)
                Release (\_SB.PCI0.SBRG.EC0.CMUT)
                If (LLess (Local0, 0x80))
                {
                    Store (Local0, LTMP) /* \_TZ_.LTMP */
                }
            }

            Return (LTMP) /* \_TZ_.LTMP */
        }

        Method (RCRT, 0, NotSerialized)
        {
            If (\_SB.PCI0.SBRG.EC0.ECAV ())
            {
                Acquire (\_SB.PCI0.SBRG.EC0.CMUT, 0xFFFF)
                Store (0xC9, \_SB.PCI0.SBRG.EC0.BRAH)
                Store (\_SB.PCI0.SBRG.EC0.ECRT, Local0)
                Release (\_SB.PCI0.SBRG.EC0.CMUT)
                If (LLess (Local0, 0x80))
                {
                    Store (Local0, \_SB.TCRT)
                }
            }

            Store (0x78, \_SB.TCRT)
        }

        Method (RPSV, 0, NotSerialized)
        {
            If (\_SB.PCI0.SBRG.EC0.ECAV ())
            {
                Acquire (\_SB.PCI0.SBRG.EC0.CMUT, 0xFFFF)
                Store (0xC9, \_SB.PCI0.SBRG.EC0.BRAH)
                Store (\_SB.PCI0.SBRG.EC0.EPSV, Local0)
                Release (\_SB.PCI0.SBRG.EC0.CMUT)
                If (LLess (Local0, 0x80))
                {
                    Store (Local0, \_SB.TPSV)
                }
            }
        }

        Method (RFAN, 1, NotSerialized)
        {
            If (\_SB.PCI0.SBRG.EC0.ECAV ())
            {
                Store (\_SB.PCI0.SBRG.EC0.ST83 (Arg0), Local0)
                If (LEqual (Local0, 0xFF))
                {
                    Return (Local0)
                }

                Store (\_SB.PCI0.SBRG.EC0.TACH (Arg0), Local0)
                Divide (Local0, 0x64, , Local0)
                Add (Local0, One, Local0)
                If (LLessEqual (Local0, 0x3C))
                {
                    Store (Local0, FANS) /* \_TZ_.FANS */
                }
                Else
                {
                    Store (FANS, Local0)
                }
            }
            Else
            {
                Store (Zero, Local0)
            }

            Return (Local0)
        }

        Method (RFSE, 0, NotSerialized)
        {
            If (\_SB.PCI0.SBRG.EC0.ECAV ())
            {
                Store (\_SB.PCI0.SBRG.EC0.ST83 (Zero), Local1)
                If (LLess (Local1, 0x80))
                {
                    If (LLess (Local1, 0x0A))
                    {
                        Store (Zero, Local0)
                    }
                    Else
                    {
                        Store (One, Local0)
                    }
                }
                ElseIf (LLess (Local1, 0xF5))
                {
                    Store (0x02, Local0)
                }
                Else
                {
                    Store (0x03, Local0)
                }
            }
            Else
            {
                Store (Zero, Local0)
            }

            Return (Local0)
        }
    }

    Scope (_SB.PCI0.SBRG.EC0)
    {
        Method (_QA2, 0, NotSerialized)  // _Qxx: EC Query, xx=0x00-0xFF
        {
            Name (PDPF, Zero)
            Acquire (CMUT, 0xFFFF)
            Store (PDST, PDPF) /* \_SB_.PCI0.SBRG.EC0_._QA2.PDPF */
            Release (CMUT)
            If (LAnd (And (PDPF, 0x02), LNotEqual (PDPF, 0x03))){}
            If (ATKP)
            {
                ^^^^ATKD.IANE (0x7B)
            }
        }
    }

    Scope (_SB.PCI0.GP17.XHC0)
    {
        Method (_S0W, 0, NotSerialized)  // _S0W: S0 Device Wake State
        {
            Return (Zero)
        }

        Device (RHUB)
        {
            Name (_ADR, Zero)  // _ADR: Address
            Device (PRT1)
            {
                Name (_ADR, One)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        0xFF, 
                        0x03, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT1._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (One, VISI) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT1._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (One, GPOS) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT1._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT1._PLD.XPLD */
                }
            }

            Device (PRT2)
            {
                Name (_ADR, 0x02)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        0xFF, 
                        0x03, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT2._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (One, VISI) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT2._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (0x02, GPOS) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT2._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT2._PLD.XPLD */
                }
            }

            Device (PRT3)
            {
                Name (_ADR, 0x03)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        0xFF, 
                        0xFF, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT3._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (Zero, VISI) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT3._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (0x03, GPOS) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT3._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT3._PLD.XPLD */
                }
            }

            Device (PRT4)
            {
                Name (_ADR, 0x04)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        Zero, 
                        Zero, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT4._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (Zero, VISI) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT4._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (0x04, GPOS) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT4._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT4._PLD.XPLD */
                }
            }

            Device (PRT5)
            {
                Name (_ADR, 0x05)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        0xFF, 
                        0x03, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT5._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (One, VISI) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT5._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (One, GPOS) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT5._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT5._PLD.XPLD */
                }
            }

            Device (PRT6)
            {
                Name (_ADR, 0x06)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        0xFF, 
                        0x03, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT6._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (One, VISI) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT6._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (0x02, GPOS) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT6._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC0.RHUB.PRT6._PLD.XPLD */
                }
            }
        }
    }

    Scope (_SB.PCI0.GP17.XHC1)
    {
        Method (_S0W, 0, NotSerialized)  // _S0W: S0 Device Wake State
        {
            Return (Zero)
        }

        Device (RHUB)
        {
            Name (_ADR, Zero)  // _ADR: Address
            Device (PRT1)
            {
                Name (_ADR, One)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        0xFF, 
                        0x09, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT1._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (One, VISI) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT1._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (0x05, GPOS) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT1._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT1._PLD.XPLD */
                }
            }

            Device (PRT2)
            {
                Name (_ADR, 0x02)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        0xFF, 
                        0x09, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT2._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (One, VISI) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT2._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (0x06, GPOS) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT2._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT2._PLD.XPLD */
                }
            }

            Device (PRT3)
            {
                Name (_ADR, 0x03)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        0xFF, 
                        0xFF, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT3._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (Zero, VISI) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT3._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (0x07, GPOS) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT3._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT3._PLD.XPLD */
                }
            }

            Device (PRT4)
            {
                Name (_ADR, 0x04)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        0xFF, 
                        0xFF, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT4._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (Zero, VISI) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT4._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (0x08, GPOS) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT4._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT4._PLD.XPLD */
                }
            }

            Device (PRT5)
            {
                Name (_ADR, 0x05)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        0xFF, 
                        0x09, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT5._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (One, VISI) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT5._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (0x05, GPOS) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT5._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT5._PLD.XPLD */
                }
            }

            Device (PRT6)
            {
                Name (_ADR, 0x06)  // _ADR: Address
                Method (_UPC, 0, Serialized)  // _UPC: USB Port Capabilities
                {
                    Name (XUPC, Package (0x04)
                    {
                        0xFF, 
                        0x09, 
                        Zero, 
                        Zero
                    })
                    Return (XUPC) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT6._UPC.XUPC */
                }

                Method (_PLD, 0, Serialized)  // _PLD: Physical Location of Device
                {
                    Name (XPLD, Package (0x01)
                    {
                        Buffer (0x14)
                        {
                            /* 0000 */  0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x61, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // a.......
                            /* 0010 */  0xFF, 0xFF, 0xFF, 0xFF                           // ....
                        }
                    })
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x40, One, VISI)
                    Store (One, VISI) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT6._PLD.VISI */
                    CreateField (DerefOf (Index (XPLD, Zero)), 0x57, 0x08, GPOS)
                    Store (0x06, GPOS) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT6._PLD.GPOS */
                    Return (XPLD) /* \_SB_.PCI0.GP17.XHC1.RHUB.PRT6._PLD.XPLD */
                }
            }
        }
    }

    Scope (_SB.I2CD)
    {
        Device (ETPD)
        {
            Name (_ADR, One)  // _ADR: Address
            Name (ETPH, Package (0x02)
            {
                "ELAN1205", 
                "ELAN1201"
            })
            Method (_HID, 0, NotSerialized)  // _HID: Hardware ID
            {
                Return ("ELAN1201")
            }

            Name (_CID, "PNP0C50" /* HID Protocol Device (I2C bus) */)  // _CID: Compatible ID
            Name (_UID, One)  // _UID: Unique ID
            Name (_S0W, 0x03)  // _S0W: S0 Device Wake State
            Method (_DSM, 4, NotSerialized)  // _DSM: Device-Specific Method
            {
                If (LEqual (Arg0, ToUUID ("3cdff6f7-4267-4555-ad05-b30a3d8938de") /* HID I2C Device */))
                {
                    If (LEqual (Arg2, Zero))
                    {
                        If (LEqual (Arg1, One))
                        {
                            Return (Buffer (One)
                            {
                                 0x03                                             // .
                            })
                        }
                        Else
                        {
                            Return (Buffer (One)
                            {
                                 0x00                                             // .
                            })
                        }
                    }

                    If (LEqual (Arg2, One))
                    {
                        Return (One)
                    }
                }
                Else
                {
                    Return (Buffer (One)
                    {
                         0x00                                             // .
                    })
                }
            }

            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (0x0F)
            }

            Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
            {
                Name (SBFI, ResourceTemplate ()
                {
                    I2cSerialBusV2 (0x0015, ControllerInitiated, 0x00061A80,
                        AddressingMode7Bit, "\\_SB.I2CD",
                        0x00, ResourceConsumer, , Exclusive,
                        )
                    GpioInt (Level, ActiveLow, Exclusive, PullUp, 0x0000,
                        "\\_SB.GPIO", 0x00, ResourceConsumer, ,
                        )
                        {   // Pin list
                            0x0009
                        }
                })
                Return (SBFI) /* \_SB_.I2CD.ETPD._CRS.SBFI */
            }
        }
    }

    Scope (_SB.PCI0.GP17.XHC1.RHUB.PRT4)
    {
        PowerResource (BTPR, 0x00, 0x0000)
        {
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (LEqual (GGOV (Zero, 0x11), One))
                {
                    Return (One)
                }
                Else
                {
                    Return (Zero)
                }
            }

            Method (_ON, 0, Serialized)  // _ON_: Power On
            {
                SGOV (Zero, 0x11, One)
            }

            Method (_OFF, 0, Serialized)  // _OFF: Power Off
            {
                SGOV (Zero, 0x11, Zero)
            }
        }

        Method (_PR0, 0, NotSerialized)  // _PR0: Power Resources for D0
        {
            Return (Package (0x01)
            {
                BTPR
            })
        }

        Method (_PR2, 0, NotSerialized)  // _PR2: Power Resources for D2
        {
            Return (Package (0x01)
            {
                BTPR
            })
        }

        Method (_PR3, 0, NotSerialized)  // _PR3: Power Resources for D3hot
        {
            Return (Package (0x01)
            {
                BTPR
            })
        }
    }

    Scope (_SB.PCI0.GPP4.WLAN)
    {
        Name (WRDY, Package (0x02)
        {
            Zero, 
            Package (0x0C)
            {
                0x07, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80
            }
        })
        Method (WRDS, 0, Serialized)
        {
            Store (STXE, Index (DerefOf (Index (WRDY, One)), One))
            Store (ST10, Index (DerefOf (Index (WRDY, One)), 0x02))
            Store (ST11, Index (DerefOf (Index (WRDY, One)), 0x03))
            Store (ST12, Index (DerefOf (Index (WRDY, One)), 0x04))
            Store (ST13, Index (DerefOf (Index (WRDY, One)), 0x05))
            Store (ST14, Index (DerefOf (Index (WRDY, One)), 0x06))
            Store (ST15, Index (DerefOf (Index (WRDY, One)), 0x07))
            Store (ST16, Index (DerefOf (Index (WRDY, One)), 0x08))
            Store (ST17, Index (DerefOf (Index (WRDY, One)), 0x09))
            Store (ST18, Index (DerefOf (Index (WRDY, One)), 0x0A))
            Store (ST19, Index (DerefOf (Index (WRDY, One)), 0x0B))
            Return (WRDY) /* \_SB_.PCI0.GPP4.WLAN.WRDY */
        }

        Name (WGDY, Package (0x02)
        {
            Zero, 
            Package (0x13)
            {
                0x07, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80, 
                0x80
            }
        })
        Method (WGDS, 0, Serialized)
        {
            Store (SD11, Index (DerefOf (Index (WGDY, One)), One))
            Store (SD12, Index (DerefOf (Index (WGDY, One)), 0x02))
            Store (SD13, Index (DerefOf (Index (WGDY, One)), 0x03))
            Store (SD14, Index (DerefOf (Index (WGDY, One)), 0x04))
            Store (SD15, Index (DerefOf (Index (WGDY, One)), 0x05))
            Store (SD16, Index (DerefOf (Index (WGDY, One)), 0x06))
            Store (SD21, Index (DerefOf (Index (WGDY, One)), 0x07))
            Store (SD22, Index (DerefOf (Index (WGDY, One)), 0x08))
            Store (SD23, Index (DerefOf (Index (WGDY, One)), 0x09))
            Store (SD24, Index (DerefOf (Index (WGDY, One)), 0x0A))
            Store (SD25, Index (DerefOf (Index (WGDY, One)), 0x0B))
            Store (SD26, Index (DerefOf (Index (WGDY, One)), 0x0C))
            Store (SD31, Index (DerefOf (Index (WGDY, One)), 0x0D))
            Store (SD32, Index (DerefOf (Index (WGDY, One)), 0x0E))
            Store (SD33, Index (DerefOf (Index (WGDY, One)), 0x0F))
            Store (SD34, Index (DerefOf (Index (WGDY, One)), 0x10))
            Store (SD35, Index (DerefOf (Index (WGDY, One)), 0x11))
            Store (SD36, Index (DerefOf (Index (WGDY, One)), 0x12))
            Return (WGDY) /* \_SB_.PCI0.GPP4.WLAN.WGDY */
        }
    }
}

