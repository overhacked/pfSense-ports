# Created by: Artyom Mirgorodskiy <artyom.mirgorodsky@gmail.com>
# $FreeBSD$

PORTNAME=	qt47-webkit
PORTVERSION=	4.7.2
DISTVERSIONSUFFIX=	-1_18.el6
PORTREVISION=	4
CATEGORIES=	www linux
MASTER_SITES=	LOCAL/tijl/atrpms:amd64,i386
DIST_SUBDIR=	atrpms

MAINTAINER=	emulation@FreeBSD.org
COMMENT=	Qt WebKit implementation (Linux CentOS ${LINUX_DIST_VER})

USES=		linux:c6
USE_LDCONFIG=	yes
USE_LINUX=	fontconfig png xorglibs sqlite3 qt qt-x11
USE_LINUX_RPM=	yes

SRC_DISTFILES=	# unavailable

DESCR=		${.CURDIR}/../../devel/linux-c6-qt47/pkg-descr

.include <bsd.port.mk>
