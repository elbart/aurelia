import { useRouter } from 'next/router';
import { MouseEvent } from 'react';

export function classNames(...classes: string[]) {
    return classes.filter(Boolean).join(' ');
};

function ActiveLink({ children, href }) {
    const router = useRouter();
    const current = router.asPath === href;

    const handleClick = (e: MouseEvent<HTMLAnchorElement>) => {
        e.preventDefault();
        router.push(href);
    };

    return (
        <a href={href} onClick={handleClick} className={classNames(
            current
                ? 'border-indigo-500 text-gray-900'
                : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700',
            'inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium'
        )} aria-current={current ? 'page' : undefined}>
            {children}
        </a>
    );
};

export default ActiveLink;