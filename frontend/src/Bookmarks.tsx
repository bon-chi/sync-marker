import React from 'react';
import {useMutation, useQuery} from '@apollo/react-hooks';
import {gql} from "apollo-boost";
import {BookmarksQuery as TBookmarksQuery} from './queries/__generated__/BookmarksQuery';
import bookmarksQuery from './queries/bookmarksQuery';
import AddBookmarkPopup from "./AddBookmarkPopup";


const Bookmarks: React.FC = () => {
  const { loading, data } = useQuery<TBookmarksQuery>(
      bookmarksQuery,
      {}
  );

  return (
      <div>
        <ul>
          {data && data.bookmarks.map(bookmark => (
            <li key={bookmark.id}>{bookmark.title}</li>
          ))}
        </ul>
        <AddBookmarkPopup/>
      </div>
  )
};

export default Bookmarks;