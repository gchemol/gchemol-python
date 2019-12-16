# [[file:~/Workspace/Programming/gchemol-python/pygchemol.note::38c3d0bf-5fd7-458f-95f7-e1c0eaf0eac3][38c3d0bf-5fd7-458f-95f7-e1c0eaf0eac3]]
# -*- coding: utf-8 -*-
#===============================================================================#
#   DESCRIPTION:  gchemol: a Graph-based chemical Objects library (since 2006)
#
#       OPTIONS:  ---
#  REQUIREMENTS:  ---
#         NOTES:  rewrite from scratch, for the 5th time
#        AUTHOR:  Wenping Guo <ybyygu@gmail.com>
#       LICENCE:  GPL version 2 or upper
#       CREATED:  <2006-08-30 Wed 16:51>
#       UPDATED:  <2018-06-05 Tue 17:16>
#===============================================================================#
# 38c3d0bf-5fd7-458f-95f7-e1c0eaf0eac3 ends here

# [[file:~/Workspace/Programming/gchemol-python/pygchemol.note::3d350342-1815-43ea-815f-32976d00448b][3d350342-1815-43ea-815f-32976d00448b]]
from __future__ import absolute_import

#from ._gchemol import hello
# 3d350342-1815-43ea-815f-32976d00448b ends here

# [[file:~/Workspace/Programming/gchemol-python/pygchemol.note::43601230-deba-40cf-8b3f-68b0649a5e41][43601230-deba-40cf-8b3f-68b0649a5e41]]
def toplevel(o):
    __all__.append(o.__name__)
    return o

__all__ = []

from .element import *
#from .atom import *
#from .bond import *
#from .molecule import *
# 43601230-deba-40cf-8b3f-68b0649a5e41 ends here
