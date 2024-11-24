; ========================================================================
;
; (C) Copyright 2023 by Molly Rocket, Inc., All Rights Reserved.
;
; This software is provided 'as-is', without any express or implied
; warranty. In no event will the authors be held liable for any damages
; arising from the use of this software.
;
; Please see https://computerenhance.com for further information
;
; ========================================================================

; ========================================================================
; LISTING 39
; ========================================================================

bits 16

; Source address calculation
mov si, bx
mov dh, al

; 8-bit immediate-to-register
mov cl, 12
mov ch, -12
